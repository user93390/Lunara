from textual.app import ComposeResult
from textual.screen import Screen
from textual.widgets import Header, Static
from textual.containers import Vertical


class HomeScreen(Screen):
    def __init__(self, username: str):
        super().__init__()
        self.username = username

    def compose(self) -> ComposeResult:
        yield Header()
        yield Vertical(
            Static(f"Welcome, {self.username}!", id="welcome-msg"),
        )

    def on_mount(self) -> None:
        self.title = "Home"
