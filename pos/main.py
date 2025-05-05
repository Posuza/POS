from kivymd.app import MDApp
from kivy.core.window import Window
from kivy.uix.screenmanager import ScreenManager
from screens.login_screen import LoginScreen
from screens.main_screen import MainScreen  # Updated import path
from screens.dashboard_screen import DashboardScreen  # Add import

Window.size = (1200, 700)

# Add screen size monitoring
def on_window_resize(instance, width, height):
    print(f'Window size changed to: {width}x{height} pixels')
    # Calculate DPI scaling
    dpi = Platform.metrics()['scalefactor'] if hasattr(Platform, 'metrics') else 1
    print(f'Screen DPI scaling: {dpi}x')
    print(f'Effective size: {width/dpi}x{height/dpi} points')

# Register window resize callback
Window.bind(size=on_window_resize)

class POSApp(MDApp):
    def build(self):
        self.theme_cls.primary_palette = "Blue"
        
        # Create screen manager
        self.sm = ScreenManager()
        
        # Create and name screens
        self.login_screen = LoginScreen(
            name='login',  # Explicit name
            login_callback=self.on_login_success
        )
        
        self.dashboard_screen = DashboardScreen(
            name='dashboard'  # Explicit name
        )
        
        self.main_screen = MainScreen()
        
        # Add screens in order
        self.sm.add_widget(self.login_screen)
        self.sm.add_widget(self.dashboard_screen)
        self.sm.add_widget(self.main_screen)
        
        # Set initial screen
        self.sm.current = 'login'
        return self.sm
    
    def handle_logout(self):
        """Central method to handle logout"""
        # Clear any app-level stored data
        self.login_screen.username.text = ""
        self.login_screen.password.text = ""
        
        # Reset screen state
        self.sm.transition.direction = 'right'
        self.sm.current = 'login'
    
    def on_login_success(self, success, role="user", username=""):
        if success:
            if role == "admin":
                self.dashboard_screen.set_username(username)
                self.sm.transition.direction = 'left'
                self.sm.current = 'dashboard'
            else:
                self.sm.transition.direction = 'left'
                self.sm.current = 'main'
                self.main_screen.set_username(username)

if __name__ == '__main__':
    POSApp().run()