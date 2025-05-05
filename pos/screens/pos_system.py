from kivy.uix.boxlayout import BoxLayout
from kivy.core.window import Window
from kivy.animation import Animation
from components.frontend.cart import CartSection
from components.frontend.products import ProductSection

class POSSystem(BoxLayout):
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.orientation = 'horizontal'
        self.padding = [20, 20, 20, 20]
        self.spacing = 0
        self.cart_visible = True
        
        # Create sections first
        self.cart_section = CartSection(size_hint_x=0.3)
        self.products_section = ProductSection(
            add_to_cart_callback=self.cart_section.add_to_cart,
            size_hint_x=0.7
        )
        
        # Cart container setup
        self.cart_container = BoxLayout(
            orientation='horizontal',
            size_hint_x=0.3,
            spacing=0
        )
        
        # Add cart section directly to container
        self.cart_container.add_widget(self.cart_section)
        
        # Add main sections
        self.add_widget(self.products_section)
        self.add_widget(self.cart_container)
        
        # Bind to window size changes
        Window.bind(size=self._on_window_resize)
        # Initial size check
        self._check_screen_size(Window.width)

    def _on_window_resize(self, instance, size):
        width = size[0]
        self._check_screen_size(width)
        
    def _check_screen_size(self, width):
        is_phone = width <= 800
        
        if is_phone and self.cart_visible:
            self.cart_visible = False
            self.products_section.size_hint_x = 0.95
            Animation(size_hint_x=0.05, duration=0.3).start(self.cart_container)
            
        elif not is_phone and not self.cart_visible:
            self.cart_visible = True
            self.products_section.size_hint_x = 0.7
            Animation(size_hint_x=0.3, duration=0.3).start(self.cart_container)
