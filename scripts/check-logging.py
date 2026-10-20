#!/usr/bin/env python3
"""Check application log messages; exclude tests and command output."""

import ast
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
CALL = re.compile(r"\b(?:tracing::)?(?:debug|info|warn|error|trace)!\s*\(|\blogger\.(?:debug|info|warn|error|fatal|trace)\s*\(")
TOKEN = re.compile(r'''"(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*'|//[^\n]*|/\*[\s\S]*?\*/|[()[\]{},]|[^"'()[\]{},/]+|.''')
PAST = re.compile(r"\b(?:\w+(?:ed|en)|was|were|failed|passed|retrieved|computed|hit|skipped|synchronized)\b")


def arguments(source, start):
    """Split call arguments while preserving nested expressions and strings."""
    depth = 1
    parts = []
    current = []
    for token in TOKEN.finditer(source, start):
        value = token.group()
        if value.startswith(('//', '/*')):
            continue
        if value in ('(', '[', '{'):
            depth += 1
        elif value in (')', ']', '}'):
            depth -= 1
            if depth == 0:
                if ''.join(current).strip():
                    parts.append(''.join(current).strip())
                return parts
        if value == ',' and depth == 1:
            parts.append(''.join(current).strip())
            current = []
        else:
            current.append(value)
    raise ValueError('Unterminated log call')


def check():
    seen = {}
    errors = []
    count = 0
    paths = list((ROOT / 'backend/crates').rglob('*.rs')) + list((ROOT / 'frontend/src').rglob('*.ts')) + list((ROOT / 'frontend/src').rglob('*.svelte'))
    for path in sorted(paths):
        if 'tests' in path.parts or path.name == 'tests.rs' or '.test.' in path.name:
            continue
        source = path.read_text().split('#[cfg(test)]')[0]
        if re.search(r'\bconsole\.(?:log|info|warn|error|debug)\(', source):
            errors.append(f'{path.relative_to(ROOT)}: use the structured logger')
        for match in CALL.finditer(source):
            count += 1
            location = f'{path.relative_to(ROOT)}:{source.count(chr(10), 0, match.start()) + 1}'
            args = arguments(source, match.end())
            candidate = args[-1] if args else ''
            try:
                message = ast.literal_eval(candidate)
            except (SyntaxError, ValueError):
                message = None
            if not isinstance(message, str) or not message or any(char in message for char in '{}%'):
                errors.append(f'{location}: message must be a nonempty static string without placeholders')
                continue
            if not PAST.search(message):
                errors.append(f'{location}: review message tense: {message}')
            if message in seen:
                errors.append(f'{location}: message duplicates {seen[message]}: {message}')
            seen[message] = location
    if errors:
        print('\n'.join(errors), file=sys.stderr)
        return 1
    print(f'Checked {count} application log call sites: static messages, distinct wording, past-tense verb, structured logger.')
    return 0


if __name__ == '__main__':
    sys.exit(check())
