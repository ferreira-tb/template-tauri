export enum Command {
  CloseWindow = 'close_window',
  FocusWindow = 'focus_window',
  ShowWindow = 'show_window',
}

export async function closeWindow() {
  await invoke(Command.CloseWindow);
}

export async function focusWindow() {
  await invoke(Command.FocusWindow);
}

export async function showWindow() {
  await invoke(Command.ShowWindow);
}
