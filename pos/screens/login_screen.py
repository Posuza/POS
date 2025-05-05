from kivymd.uix.screen import MDScreen
from kivy.uix.boxlayout import BoxLayout
from kivymd.uix.button import MDRaisedButton
from kivymd.uix.textfield import MDTextField
from kivy.metrics import dp

class LoginScreen(MDScreen):
    def __init__(self, login_callback, **kwargs):
        super().__init__(**kwargs)
        self.name = 'login'
        self.login_callback = login_callback
        
        # Create main layout
        layout = BoxLayout(
            orientation='vertical',
            spacing=dp(20),
            padding=dp(50),
            size_hint=(None, None),
            size=(dp(300), dp(400)),
            pos_hint={'center_x': .5, 'center_y': .5}
        )
        
        # Username field
        self.username = MDTextField(
            hint_text="Username",
            helper_text="Enter your username",
            helper_text_mode="on_error",
            pos_hint={'center_x': .5}
        )
        
        # Password field
        self.password = MDTextField(
            hint_text="Password",
            helper_text="Enter your password",
            helper_text_mode="on_error",
            password=True,
            pos_hint={'center_x': .5}
        )
        
        # Login button
        login_button = MDRaisedButton(
            text="LOGIN",
            size_hint=(None, None),
            width=dp(200),
            pos_hint={'center_x': .5},
            on_release=self.validate_login
        )
        
        # Add widgets to layout
        layout.add_widget(self.username)
        layout.add_widget(self.password)
        layout.add_widget(login_button)
        
        self.add_widget(layout)
    
    def validate_login(self, instance):
        username = self.username.text.strip()
        password = self.password.text.strip()
        
        if not username or not password:
            self.username.helper_text = "Username required"
            self.password.helper_text = "Password required"
            return
            
        # Admin credentials check
        ADMIN_USERS = {
            'admin': '123',
            'manager': '123'
        }
        
        if username in ADMIN_USERS and password == ADMIN_USERS[username]:
            self.login_callback(True, "admin", username)
        else:
            # Regular user login
            self.login_callback(True, "user", username)
