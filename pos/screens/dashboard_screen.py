from kivymd.uix.screen import MDScreen
from kivy.uix.boxlayout import BoxLayout
from kivymd.uix.card import MDCard
from kivymd.uix.label import MDLabel
from kivymd.uix.button import MDRaisedButton
from kivy.metrics import dp
from kivymd.app import MDApp
from kivy.graphics import Color, Rectangle
from components.backend.display import Display  # Remove HeaderDisplay from import
from components.backend.sidebar import Sidebar

class StatCard(MDCard):
    def __init__(self, title, value, **kwargs):
        super().__init__(**kwargs)
        self.orientation = 'vertical'
        self.padding = dp(15)
        self.spacing = dp(10)
        self.size_hint = (None, None)
        self.size = (dp(200), dp(150))
        self.elevation = 2

        # Add title and value
        self.add_widget(MDLabel(
            text=title,
            theme_text_color="Secondary",
            font_style="Caption"
        ))
        self.add_widget(MDLabel(
            text=str(value),
            theme_text_color="Primary",
            font_style="H4"
        ))

class DashboardScreen(MDScreen):
    def __init__(self, **kwargs):

        super().__init__(**kwargs)
        self.name = 'dashboard'
        
        # Create main layout
        main_layout = BoxLayout(orientation='horizontal')
        
        # Create sidebar with callback
        self.sidebar = Sidebar(switch_view_callback=self.switch_view, parent=None)
        
        # Create display
        self.display = Display(add_to_cart_callback=self.add_to_cart)
        
        # Add widgets to main layout
        main_layout.add_widget(self.sidebar)
        main_layout.add_widget(self.display)
        
        # Add main layout to screen
        self.add_widget(main_layout)

    def switch_view(self, view_name,icon):
        print(f"DashboardScreen switching to view: {view_name}")
        self.display.handle_view_switch(view_name,icon)
    
    def set_username(self, username):
        self.display.set_username(username)
    
    def add_to_cart(self, product):
        # Implement cart functionality
        pass
    
    def set_admin_name(self, username):
        """Alias for set_username for better semantic clarity"""
        self.set_username(username)
