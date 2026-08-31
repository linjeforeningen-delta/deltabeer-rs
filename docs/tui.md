# TUI

## Overview

The DeltaBeer TUI is the terminal presentation and input layer.
It owns presentation state, page state, dialog state, and local input state.
It communicates with the server through the HTTP API and does not access the database directly.

## Pages

### Home

Home displays the welcome message and tells the operator that a card can be scanned at any time to open a user.
It has no page-specific key handler.

### Users

Users displays the complete user collection returned by the API as a searchable and sortable table.
The table includes name, username, program, card, role, birthdate, balance, and spent amount.
Selection moves through the currently visible rows.
The page has local search, selection, sorting, and refresh actions described below.
The current Enter-to-open-user handler is commented out, so Enter does not open the selected user.

### Transactions

Transactions displays the transaction table headings.
The current table has no rows and the page has no page-specific key handler.

### Stats

Stats displays the system summary returned by the API: user count, total balance, total spent, and transaction count.
It has a page-specific refresh action described below.

## Key handling and precedence

For a terminal key event, processing occurs in this order:

1. `Runtime::handle_event` handles `Ctrl-Q` immediately and dispatches `Quit`.
2. Other key events go to the scanner layer, which may wait for more digits, emit a completed scan, or release ordinary key events.
3. Released ordinary keys are checked for keyboard-global bindings, currently `Ctrl-L`, which toggles the language.
4. If a dialog is active, the active dialog receives the key.
5. If the dialog consumes or translates the key, processing stops; otherwise the key is passed to the active page.
6. If the page consumes or translates the key, processing stops; otherwise base navigation handles it.
7. Base handling is disabled whenever any dialog is present.
8. With no dialog, `1`–`4` navigate to Home, Users, Transactions, and Stats, respectively, and `Esc` opens the application menu.

`Ctrl-Q` therefore has precedence over scanner handling.
`Ctrl-L` is below scanner handling but above dialog, page, and base handling.
An active dialog handles `Esc` through the default dialog behavior, which emits `CloseDialog`.
Dialog `Esc` is not base-handled, and base navigation does not run while a dialog is present.

## Page key bindings

| Page         | Key                             | Behavior                                                 |
| ------------ | ------------------------------- | -------------------------------------------------------- |
| Home         | —                               | No page-specific action.                                 |
| Users        | `/`                             | Activate search.                                         |
| Users        | Text characters                 | Append to the search while search is active.             |
| Users        | `Backspace`                     | Remove the last search character while search is active. |
| Users        | `Enter`, `Esc`, or `/`          | Deactivate search and retain the query.                  |
| Users        | `Up` / `Down`                   | Move the selected visible user.                          |
| Users        | `Tab` / `Shift-Tab` / `BackTab` | Move to the next or previous sort field.                 |
| Users        | `s`                             | Toggle ascending/descending sort order.                  |
| Users        | `r`                             | Reload all users from the API.                           |
| Users        | `Enter` when search is inactive | No action; the selected-user opening code is disabled.   |
| Transactions | —                               | No page-specific action.                                 |
| Stats        | `r`                             | Reload statistics from the API.                          |

The Users search matches lowercased text across user fields, including identifier, name, username, program, card number, role, birthdate, comments, balance, and spent amount.
Sorting is by the selected field, with user ID as a tie-breaker.

## Scanner input model

The scanner buffer accepts only `KeyCode::Char` events whose characters are ASCII decimal digits.
An `Enter` event terminates a non-empty digit buffer and emits the buffered digits as one card identifier.
The scanner does not accept letters, Unicode digits, function keys, or other key codes as scan characters.

The configured `scanner_max_gap_ms` is used by the periodic flush to decide whether the time since the last buffered digit has exceeded the maximum scan gap.
While the gap has not expired, buffered digits remain pending.
When the gap expires, the buffered digit events are released as ordinary keyboard events.
If a non-scanner key arrives while digits are buffered, the digits and that key are released immediately as ordinary keyboard events rather than producing a scan.
Released events are passed through the normal keyboard-global, dialog, page, and base mapping in their original order.
The expected reader contract is: the TUI expects a keyboard-emulating reader that emits ASCII decimal digits followed by `Enter` as terminal key events.

### Hardware and OS boundary

The TUI consumes terminal/input events delivered by the terminal event layer.
It does not discover, configure, reset, or repair a physical reader.
Missing, truncated, or corrupt events that are already absent or malformed outside DeltaBeer are outside the TUI's scanner grouping logic.
For reader troubleshooting, see [operations.md](operations.md).

## Card lookup and spending

A completed card scan becomes a `LookupUser` request.
The API client first resolves the identifier with `/v1/users/resolve/{identifier}` and then fetches the resolved user with `/v1/users/{user_id}`.
The result opens or updates the user dialog.
Entering an amount and pressing `Enter` sends a spend request for that user.
Lookup and spending are public operations in the TUI API request model and do not require an admin token.

Top-up, user creation, user updates, granting admin privileges, and revoking admin privileges are protected operations.

## Users page data behavior

Entering the Users page and pressing `r` requests the full user list through `GET /v1/users`.
The TUI keeps that list in memory.
Filtering is local and case-insensitive.
Sorting is local.
Selection is stored as a `UserId`, not as a row number, and is reconciled when the visible set changes.
The TUI does not send server-side search or pagination parameters.

## Admin authentication and sessions

The TUI has two distinct authorization patterns.
For a protected operation without an active admin session, the TUI stores one pending API request in memory and opens the administrator-authentication dialog.
The entered password is sent with the admin user ID to obtain a single-use token.
That single-use token authorizes the pending protected request as a bearer token.
The token is not converted into a lasting local session by this flow.

An explicit admin-session login first obtains a single-use token from the password endpoint, then exchanges it for a session token through the session endpoint.
The TUI stores the session token and its admin user ID only in in-memory `AuthState`.
Protected requests then use the session token as bearer authorization.
Logging out sends the session token to the API, and a successful result changes the local auth state to normal.
Closing the outermost dialog clears the active admin context and requests session logout.
When a different admin user is opened, or a non-admin user is opened while an admin session is active, the previous session is ended before continuing with the new context.

## Dialog stack

`Push` adds a dialog above the current top dialog.
`ReplaceTop` removes the current top dialog and adds the new dialog.
`Reset` clears the entire stack and adds the new dialog.
`CloseDialog` pops only the active dialog.
Successful administrative operations such as top-up and user management close back to the admin menu when that menu remains in the stack.
Clearing the stack removes all dialogs.
Idle transition clears the entire dialog stack.

## Splash and idle behavior

The startup splash duration is a fixed 1,250 milliseconds.
The configured `tui.idle_splash_after_seconds` controls when an active UI becomes idle.
Idle displays the splash and clears all dialogs.
Any terminal event handled by the runtime activates the UI again, including a key event, while input-buffer timeout processing alone does not activate it.
The configured `tui.event_poll_interval_ms` controls terminal event polling and scanner flush cadence.

## Localization

The bundled locales are English (`en`) and Norwegian Bokmål (`nb`).
The configured locale is validated against the bundled locale list during startup, and startup fails for an unsupported locale.
`Ctrl-L` toggles the locale at runtime.
The application menu also provides English and Norwegian Bokmål choices.

## Request/result/state flow

Input is converted into a message.
The application update layer turns API-request messages into API commands or local state changes.
The runtime executes API commands over HTTP.
HTTP responses are decoded into API results or errors and returned as messages.
The update layer applies those messages to authentication, page, dialog, status, and other application state.
The renderer displays the resulting state.

## Known limitations

The Transactions page currently renders an empty table and has no page-specific actions.
The Users page's selected-user Enter action is intentionally incomplete because its handler is commented out.
Scanner grouping supports only ASCII digit key events followed by `Enter`.
Other reader output is ordinary terminal input or falls outside the TUI's control.
