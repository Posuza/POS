from kivymd.uix.screen import MDScreen
from kivy.uix.boxlayout import BoxLayout
from kivy.metrics import dp
from components.frontend.cart import CartSection
from components.frontend.productDisplay import Display  # Changed from ProductDisplay to Display

class MainScreen(MDScreen):
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.name = 'main'
        
        # Create horizontal layout for split view
        main_layout = BoxLayout(
            orientation='horizontal',
            spacing=dp(5),
            padding=[dp(5), dp(5), dp(5), dp(5)]
        )
        
        # Create products display
        self.product_display = Display(
            add_to_cart_callback=self.add_to_cart,
            size_hint_x=0.65
        )
        
        # Create cart section
        self.cart_section = CartSection()
        self.cart_section.size_hint_x=0.35
        
        # Add widgets to main layout
        main_layout.add_widget(self.product_display)
        main_layout.add_widget(self.cart_section)
        
        # Add main layout to screen
        self.add_widget(main_layout)
    
    def set_username(self, username):
        """Update username in product display"""
        self.product_display.set_username(username)
    
    def add_to_cart(self, serial, product, price):
        """Handle adding products to cart"""
        self.cart_section.add_to_cart(serial, product, price)
