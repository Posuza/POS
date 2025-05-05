from kivy.uix.boxlayout import BoxLayout
from kivy.uix.button import Button
from kivy.uix.label import Label
from kivy.metrics import dp
from kivy.graphics import Color, RoundedRectangle
from kivy.core.window import Window

class ProductCard(BoxLayout):
    def __init__(self, serial, product, price, quantity, callback, **kwargs):
        super().__init__(**kwargs)
        self.orientation = 'vertical'
        self.callback = callback  # Store callback as instance variable
        
        # Determine screen type
        width = Window.width
        is_phone = width <= 800
        is_tablet = 800 < width <= 1024
        
        # Adjust card height
        self.size_hint_y = None
        self.height = dp(90) if is_phone else dp(100) if is_tablet else dp(120)
        
        # Adjust padding and spacing
        self.padding = dp(1) if is_phone else dp(3) if is_tablet else dp(5)
        self.spacing = dp(1) if is_phone else dp(3) if is_tablet else dp(5)

        # Adjust font sizes
        name_font_size = '11sp' if is_phone else '13sp' if is_tablet else '16sp'
        price_font_size = '13sp' if is_phone else '15sp' if is_tablet else '18sp'
        serial_font_size = '9sp' if is_phone else '10sp' if is_tablet else '12sp'
        
        # Adjust qty badge size
        qty_size = dp(20) if is_phone else dp(30) if is_tablet else dp(40)
        qty_font = '12sp' if is_phone else '13sp' if is_tablet else '14sp'

        # Create product container
        self.product_container = BoxLayout(
            orientation='vertical',
            padding=[dp(1) if is_phone else dp(3) if is_tablet else dp(5), dp(3)],
            spacing=dp(1)
        )
        
        # Add background with rounded corners
        with self.product_container.canvas.before:
            Color(0.2, 0.6, 0.8, 1)
            self.rect = RoundedRectangle(
                pos=self.product_container.pos,
                size=self.product_container.size,
                radius=[dp(10),]
            )
        self.product_container.bind(pos=self._update_rect, size=self._update_rect)
        
        # Stock quantity badge in top-right corner
        qty_container = BoxLayout(
            size_hint=(None, None),
            size=(qty_size, qty_size * 0.6),
            pos_hint={'right': 1.02, 'top': 1}
        )
        
        with qty_container.canvas.before:
            Color(0.95, 0.95, 0.95, 0.9)  # Light background
            self.qty_rect = RoundedRectangle(
                pos=qty_container.pos,
                size=qty_container.size,
                radius=[dp(12.5),]
            )
        qty_container.bind(pos=self._update_qty_rect, size=self._update_qty_rect)
        
        qty_label = Label(
            text=str(quantity),
            font_size=qty_font,
            bold=True,
            color=(0.2, 0.2, 0.2, 1)  # Dark text
        )
        qty_container.add_widget(qty_label)
        
        # Add serial number label
        self.serial_label = Label(
            text=serial,
            font_size=serial_font_size,
            color=(1, 1, 1, 0.7),
            size_hint_y=0.2,
            halign='center',  # Changed from 'left'
            valign='middle'   # Added valign
        )
        self.serial_label.bind(size=self._update_label)  # Add binding
        
        # Product name with better styling
        self.name_label = Label(
            text=product,
            font_size=name_font_size,
            bold=True,
            color=(1, 1, 1, 1),
            size_hint_y=0.6,
            halign='center',
            valign='middle'
        )
        self.name_label.bind(size=self._update_label)
        
        # Price with currency symbol
        self.price_label = Label(
            text=f'${price:.2f}',
            font_size=price_font_size,
            color=(1, 1, 1, 0.9),
            size_hint_y=0.4,
            bold=True,
            halign='center',  # Added halign
            valign='middle'   # Added valign
        )
        self.price_label.bind(size=self._update_label)  # Add binding
        
        # Add all elements to container
        self.product_container.add_widget(qty_container)
        self.product_container.add_widget(self.serial_label)
        self.product_container.add_widget(self.name_label)
        self.product_container.add_widget(self.price_label)
        
        # Update callback binding to handle touch event properly
        self.product_container.bind(
            on_touch_down=lambda instance, touch: self.handle_touch(touch, serial, product, price)
            if self.collide_point(*touch.pos) else False
        )
        
        self.add_widget(self.product_container)

    def _update_rect(self, instance, value):
        self.rect.pos = instance.pos
        self.rect.size = instance.size
    
    def _update_qty_rect(self, instance, value):
        self.qty_rect.pos = instance.pos
        self.qty_rect.size = instance.size
        
    def _update_label(self, instance, value):
        instance.text_size = value

    def handle_touch(self, touch, serial, product, price):
        """Handle touch event and call callback with product data"""
        if self.collide_point(*touch.pos):
            if self.callback:  # Check if callback exists
                self.callback(serial, product, price)
            return True
        return False
