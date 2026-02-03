# Plan to Update the Terminal UI

This plan outlines the steps to modify the `ratatui` interface. The goal is to split the terminal view vertically. The top section will continue to display the real-time bar chart, and a new bottom section will be added to show server messages.

## 1. Modify the UI Layout in `src/ui.rs`

The current layout in `src/ui.rs` uses a single horizontal `Layout` which fills the entire screen. This needs to be changed to a vertical layout to accommodate the message panel.

-   **Action:** Modify the `render` function in `src/ui.rs`.
-   **Details:**
    -   Change `Layout::direction` from `Direction::Horizontal` to `Direction::Vertical`.
    -   Define two `Constraint`s to split the screen. A possible split would be `[Constraint::Percentage(80), Constraint::Percentage(20)]`. The first constraint will be for the bar chart, and the second for the server messages.
    -   Render the `BarChart` in the top area (the first item in the split layout).

## 2. Create a Server Message Panel in `src/ui.rs`

A new widget needs to be created to display the server messages in the bottom part of the screen.

-   **Action:** Add a new widget to the `render` function in `src/ui.rs`.
-   **Details:**
    -   Use a `Paragraph` widget from `ratatui` to display the text messages.
    -   The `Paragraph` should be rendered in the bottom area (the second item in the split layout).
    -   The text for the `Paragraph` will come from a new shared state that we'll create in the next step.
    -   Style the `Paragraph` with a `Block` to give it a title and borders, visually separating it from the chart.

## 3. Capture and Share Server Messages

Currently, server-side events (like new client connections or received messages) are handled in `main.rs`, but they are not displayed in the UI. We need a mechanism to pass these messages to the `render` function.

-   **Action:** Implement a shared state for server messages in `src/main.rs`.
-   **Details:**
    -   Create a new global static variable, similar to `CLIENT_DATA`. Let's name it `SERVER_LOGS`.
    -   The type for `SERVER_LOGS` should be `Mutex<Vec<String>>` to hold a thread-safe, growable list of log messages.
    -   In the `handle_client` async function in `main.rs`, and in the `Server` command handler where the listener is created, push relevant messages (e.g., "new client!", "Received from client: ...") into the `SERVER_LOGS` vector.
    -   In the `render` function in `src/ui.rs`, lock the `SERVER_LOGS` mutex, read the latest messages, and pass them to the `Paragraph` widget created in step 2. To avoid the log growing indefinitely, we could consider only showing the last N messages.
