/**
 * Whether a navigation has been running long enough to be worth showing.
 *
 * A filter change reloads the board through the URL, and flipping the controls
 * into a loading state the instant it starts means a fast answer is three
 * visible changes in a row: the state goes on, the rows swap, the state comes
 * off. Below the delay nothing moves, so the common case is a single repaint
 * of the rows, and a wait only announces itself once it is long enough that
 * silence would read as a broken click.
 *
 * Call it while a component is initialising, so its effect is cleaned up with
 * the component.
 */
export function slowNavigation(active: () => boolean, delayMs = 150) {
  let slow = $state(false);

  $effect(() => {
    if (!active()) {
      slow = false;
      return;
    }

    const timer = setTimeout(() => (slow = true), delayMs);
    return () => clearTimeout(timer);
  });

  return {
    get current() {
      return slow;
    },
  };
}
