# The FinalRep app, and how to get results out of it

Notes from reverse engineering `app.final-rep.com` on 2026-08-29, while importing the
Chilean Open 2026 and the Costa Rican Open 2026. Everything below was observed
directly, not inferred from documentation, because there is no public documentation.

## Why screenshots were the starting point

The app is **Flutter web rendering through CanvasKit**. The results table is painted
into a `<canvas>`, so there is no DOM to parse, no table markup, no text nodes. Every
technique that works on a server-rendered page fails here, and a screenshot really is
the only thing a browser can hand you without touching the network layer.

That makes the API the only route to structured data.

## The API

Base is `https://api.final-rep.com`, split into versioned namespaces. Two are known:

| Namespace | Seen |
| --- | --- |
| `users-api/v1` | `GET /users-api/v1/users/current` |
| `events-api/v1` | `GET /events-api/v1/events/{eventId}/statistics`<br>`GET /events-api/v1/events/{eventId}/current/groups` |

Responses are HAL-flavoured: the payload sits under `_embedded`, and `_meta` carries a
`request_id`. Ids everywhere are KSUIDs, 27 characters of base62
(`39mYBtfbYwevgBazsYfS4LNymd5`).

### Auth

A cookie, not a bearer header:

```
Cookie: access=<JWT>
```

The JWT is **ES512**, `kid: "access"`, issued by `Final Rep`, and its claims are
`exp, iat, iss, jti, nbf, prms, sub`. `sub` is the user's KSUID and `prms` is a
permissions array, empty for an ordinary account.

It lives **20 minutes** (`exp - iat == 1200`). Plan around that: capture, then work
quickly, or re-capture. There is presumably a refresh endpoint the app calls, but it
was not in the capture.

The server also honours `If-None-Match`, so the app is doing ETag revalidation.

### Headers that were actually sent

```
Accept: application/json
content-type: application/json
Origin: https://app.final-rep.com
Referer: https://app.final-rep.com/
Cookie: access=<JWT>
```

Which of these are strictly required was not tested. Sending all of them works.

### Route probing, and how to read the errors

The two failure modes are distinguishable, which makes blind probing productive:

| Response | Means |
| --- | --- |
| `404 page not found` (plain text) | the route does not exist |
| `500 {"errors":["key not found"]}` | the route exists, the path segment was parsed as an id and no record matched |

So `GET /events-api/v1/events/anything` returning the 500 tells you
`/events-api/v1/events/{id}` is a real route, while `/events-api/v1/events` returning
404 tells you there is **no list endpoint** at that path. No listing route was found;
the event id has to come from the app.

## `GET /events-api/v1/events/{eventId}/statistics`

The whole meet in one response. About 68 KB for a nine-athlete meet.

```
_embedded
  event_name        "Costa Rican Open"
  event_type        "Calisthenics ONERM"
  meta_statistics   total_attempts, finished, unfinished, success, failure,
                    event_records[] { name, amount, weight, user_id }
  group_stats[]     one per weight class
    sort_by_ris     false, so the class is ranked on total
    group           { id, name: "Male -66kg", starters[], rounds[], default, disabled }
    athlete_stats[]
      user          { id, tag, name, image_url, country, gender, team, social_media, ... }
      place         1-based, and -1 for a disqualified athlete
      total         number
      ris           full precision, e.g. 90.00945738281455
      disqualified  bool
      finished      bool
      date
      attempts      { "Muscle Up" | "Pull/Chin Up" | "Dip" | "Squat": [ ... ] }
        attempt     1, 2, 3
        weight      number
        success     bool
        finished    bool
        previous    id of the preceding attempt, or ""
        exercise_id, round_id, flight_id, group_id, user_id, event_id
        created, expires, execution_time
```

`GET .../current/groups` is much smaller and returns per-class status
(`{"Male -66kg": "live", ...}`) keyed both by name and by group id.

### Mapping onto `entries.csv`

Almost everything the canonical format wants is here:

| entries.csv | API |
| --- | --- |
| `Sex`, `WeightClassKg` | parse `group.name`, e.g. `"Male -66kg"` |
| `FirstName`, `LastName` | `user.name`, untruncated |
| `Country` | `user.country`, but see below |
| `Ris` | `athlete_stats.ris` |
| `Status` | `disqualified` |
| `<Movement>{1,2,3}Kg` | `attempts[movement][].weight` plus `.success` for the `x` suffix |

`attempts` is keyed by the movement's display name, and those names line up with the
project's four movements directly. Sort by `attempt`, do not trust array order.

### Bodyweight is not there

Searched the entire payload for any bodyweight, weight-class or `bw` field: **nothing**.
FinalRep does not appear to record bodyweight at all, so this is not a UI omission we
can route around. Meets sourced from FinalRep are `Ris`-only rows, permanently, unless
the organiser supplies a separate scoresheet.

### `country` is not always ISO alpha-2

Mostly it is a lowercase ISO 3166-1 alpha-2 code (`"cr"`, `"cl"`). But some athletes
carry **`"419"`**, which is the UN M49 code for *Latin America and the Caribbean*.

The app cannot draw a flag for a region, which is exactly why those athletes render with
a blank flag cell or a grey globe. So a missing flag does not mean "unknown", it means
the athlete picked a region rather than a country. Either way it is not importable as
alpha-2, and the host-country default still applies, but the record should say the
athlete declined to name a country rather than that the data was absent.

## Capturing a session

The Network tab is the only reliable way in, because the id has to come from the app.

1. Open `app.final-rep.com`, sign in, navigate to the event's results.
2. DevTools, Network tab. Filter on `final-rep` by **domain**, not by the XHR/Fetch
   type buttons, since Flutter's requests are not always tagged the way you expect.
3. **Reload the page** while sitting on the results view. The app caches, so clicking
   between weight classes may fire nothing at all.
4. Right-click the request list, **Save All As HAR**.

The HAR carries every response body and the cookie, so one file is enough to work
offline afterwards. It also contains a live credential, so keep it out of the repo.

Pulling the payloads back out:

```python
import json
har = json.load(open('capture.har'))
for e in har['log']['entries']:
    if 'statistics' in e['request']['url'] and e['request']['method'] == 'GET':
        stats = json.loads(e['response']['content']['text'])
```

"Copy as cURL" on a single request works too, and is the fastest way to get a shell
usable against the API, but you have to find the right request yourself.

If the Network tab is unhelpful, driving Chrome over CDP works: relaunch with
`--remote-debugging-port=9222`, attach, enable the `Network` domain, and dump every
response body via `Network.getResponseBody`. Node 22+ has both `fetch` and `WebSocket`
built in, so this needs no dependencies.

## Where the token hides, if you want it directly

Flutter's `shared_preferences` maps to `localStorage` with every key prefixed
`flutter.`. `flutter_secure_storage` and Hive both land in **IndexedDB** instead, which
is why the Local/Session Storage panes can look empty. In this app auth turned out to be
a plain cookie, so none of that was needed.

## Verification: the screenshot pipeline is sound

The Costa Rican Open was imported from screenshots first and checked against this API
afterwards. Nine athletes, every attempt weight, every success flag, every RIS and every
disqualification: **zero discrepancies**.

So the colour legend documented in the `extract-competition` skill (green made, orange
boxed counted, red failed) is confirmed correct against the underlying data, and reading
screenshots remains a legitimate fallback when no session is available. The API's
advantages are that it is faster, carries untruncated names, exposes full-precision RIS
and Instagram handles, and removes transcription risk entirely.

## Limits worth respecting

This is an ordinary account reading its own view of published results, a handful of
requests per meet. Keep it that way: no crawling, no parallel hammering, and re-capture
by hand rather than automating a login. A sanctioned token is better than a scraped one
for something that will run dozens of times, and `support@final-rep.com` is a reasonable
place to ask, since a results aggregator is a friendly use case.

## Open questions

- No event listing endpoint was found, so event ids must come from the app UI.
- The refresh-token flow was not captured.
- `event_type` was `"Calisthenics ONERM"` here; other values are unknown.
- `lor_multiplier` on the group object was an empty array and its meaning is unknown.
