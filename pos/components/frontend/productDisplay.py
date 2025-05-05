from kivy.uix.boxlayout import BoxLayout
from components.frontend.products import ProductSection
from kivy.metrics import dp

class Display(BoxLayout):  # Changed back to Display to match imports
    def __init__(self, add_to_cart_callback, **kwargs):
        super().__init__(**kwargs)
        self.orientation = 'vertical'
        self.size_hint_x = 1
        self.spacing = dp(2)
        self.padding = [dp(5), dp(5), dp(5), dp(5)]
        
        # Create products section
        self.products_section = ProductSection(
            add_to_cart_callback=add_to_cart_callback,
            size_hint_x=1,
            spacing=dp(2),
            padding=[dp(2), dp(2), dp(2), dp(2)]
        )
        
        # Add widgets
        self.add_widget(self.products_section)
    
    def set_username(self, username):
        """Update username in products section"""
        self.products_section.set_username(username)
    
    def logout(self, instance):
        """Handle logout"""
        self.products_section.logout(instance)

# Remove duplicate ProductDisplay class as it's essentially the same as Display
