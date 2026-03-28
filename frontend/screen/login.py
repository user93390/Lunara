from textual.app import ComposeResult
from textual.screen import Screen
from textual.widgets import Header, Input, Button
from textual.containers import Vertical

login_id = "login"
 
class LoginScreen(Screen):
    def compose(self) -> ComposeResult:
        yield Header()
        yield Vertical(
            Input(id="username", placeholder="Username", type="text"),
            Input(id="password", placeholder="Password", type="text"),
            Button("Login", id=login_id),
        )

    def on_mount(self) -> None:
        self.title = "Login"

    def on_button_pressed(self, event: Button.Pressed) -> None:
        if event.button.id == login_id:
            username_input = self.query_one("#username", Input)
            username = username_input.value

            from .home import HomeScreen
            self.app.push_screen(HomeScreen(username))
