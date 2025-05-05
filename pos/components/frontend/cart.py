from kivy.uix.boxlayout import BoxLayout
from kivy.uix.button import Button
from kivy.uix.label import Label
from kivy.uix.scrollview import ScrollView
from kivy.metrics import dp
from kivy.graphics import Color, RoundedRectangle, Rectangle
from kivy.core.window import Window
from kivymd.uix.label import MDLabel
from .cart_item import CartItem  # Import CartItem from new file
from .checkout_popup import CheckoutPopup  # Add this import at the top

class CartSection(BoxLayout):
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.orientation = 'vertical'
        self.spacing = dp(5)
        self.padding = [dp(10), dp(10), 0, 0]  # [left, top, right, bottom] - added top padding
        self.cart_items = {}
        self.total = 0.0

        # Add background
        with self.canvas.before:
            Color(0.95, 0.95, 0.95, 1)
            self.rect = Rectangle(pos=self.pos, size=self.size)
        self.bind(pos=self._update_rect, size=self._update_rect)

        # Add header and column titles at the top
        self.add_widget(self.create_header())
        self.add_widget(self.create_column_titles())

        # Upper Section - Cart Items (80% height)
        self.cart_upper = BoxLayout(
            orientation='vertical',
            size_hint_y=0.8,
            spacing=dp(5)
        )
        
        self.create_items_area()
        self.add_widget(self.cart_upper)

        # Lower Section - Checkout Area (20% height)
        self.cart_lower = BoxLayout(
            orientation='vertical',
            size_hint_y=0.2,
            spacing=dp(10),
            padding=[10, 5, 10, 10]
        )
        
        # Total Section with Label and Amount
        total_container = BoxLayout(
            orientation='horizontal',
            size_hint_y=0.4,
            spacing=dp(10),
            padding=[0, 0, dp(20), 0]  # Add right padding
        )
        
        # Amount only (removed Total: label)
        self.total_amount = Label(
            text='$0.00',
            size_hint_x=1.0,  # Take full width
            font_size='28sp',
            color=(0.2, 0.6, 0.8, 1),
            bold=True,
            halign='right',
            valign='middle'
        )
        
        total_container.add_widget(self.total_amount)
        self.cart_lower.add_widget(total_container)
        
        # Checkout Button
        checkout_btn = Button(
            text='Checkout',
            size_hint_y=0.6,
            background_color=(0.2, 0.8, 0.2, 1),
            background_normal='',
            font_size='20sp',
            bold=True
        )
        
        with checkout_btn.canvas.before:
            Color(0.2, 0.8, 0.2, 1)
            self.checkout_rect = RoundedRectangle(
                pos=checkout_btn.pos,
                size=checkout_btn.size,
                radius=[25,]
            )
        checkout_btn.bind(pos=self.update_checkout_rect, size=self.update_checkout_rect)
        checkout_btn.bind(on_press=self.checkout)
        
        self.cart_lower.add_widget(checkout_btn)
        self.add_widget(self.cart_lower)

        # Store header labels as instance variables
        self.cart_header = None
        self.clear_btn = None
        
        # Bind window size changes
        Window.bind(size=self.on_window_resize)

    def _update_rect(self, instance, value):
        self.rect.pos = instance.pos
        self.rect.size = instance.size

    def add_item(self, product):
        """Deprecated - use add_to_cart instead"""
        pass

    def add_to_cart(self, serial, product, price):
        """Add item to cart with all product details"""
        if product in self.cart_items:
            current_price, current_qty = self.cart_items[product]
            self.cart_items[product] = (price, current_qty + 1)
        else:
            self.cart_items[product] = (price, 1)
        
        self.update_total()  # Update total first
        self.update_display()

    def update_display(self):
        self.cart_items_layout.clear_widgets()
        for product, (price, quantity) in self.cart_items.items():
            cart_item = CartItem(
                product=product,
                price=price,
                quantity=quantity,
                delete_callback=self.remove_item
            )
            self.cart_items_layout.add_widget(cart_item)
        self.total_amount.text = f'${self.total:.2f}'

    def _get_cart_title_size(self):
        """Calculate cart title size based on window width"""
        base_size = 40  # Base height for large screens
        if Window.width <= 800:
            return dp(25)  # Smaller height for small screens
        elif Window.width <= 1024:
            return dp(32)  # Medium height for tablet screens
        return dp(base_size)

    def create_header(self):
        header_container = BoxLayout(
            orientation='horizontal',
            size_hint_y=None,
            height=self._get_cart_title_size(),  # Dynamic height
            spacing=dp(10)
        )
        
        # Store references and set initial font sizes
        self.cart_header = Label(
            text='Cart',
            size_hint_x=0.5,
            font_size=f'{self._get_responsive_font_size(32)}sp',  # Increased base size
            bold=True,
            color=(0.2, 0.2, 0.2, 1),
            halign='left'
        )
        
        self.clear_btn = Button(
            text='Clear All',
            size_hint_x=0.3,
            background_color=(0.8, 0.2, 0.2, 1),
            background_normal='',
            font_size='12sp',  # Fixed font size
            bold=True
        )
        
        with self.clear_btn.canvas.before:
            Color(0.8, 0.2, 0.2, 1)
            self.clear_rect = RoundedRectangle(
                pos=self.clear_btn.pos,
                size=self.clear_btn.size,
                radius=[20,]
            )
        self.clear_btn.bind(pos=self.update_clear_rect, size=self.update_clear_rect)
        self.clear_btn.bind(on_press=self.clear_cart)
        
        header_container.add_widget(self.cart_header)
        header_container.add_widget(self.clear_btn)
        return header_container

    def _get_responsive_font_size(self, base_size):
        """Calculate font size based on screen width"""
        if Window.width <= 600:  # Very small screens
            return base_size * 0.4
        elif Window.width <= 800:  # Small screens
            return base_size * 0.5
        elif Window.width <= 1024:  # Medium/tablet screens
            return base_size * 0.65
        return base_size * 0.8  # Large screens but still scaled down

    def on_window_resize(self, instance, value):
        """Update only cart title font size when window resizes"""
        if self.cart_header:
            self.cart_header.font_size = f'{self._get_responsive_font_size(32)}sp'
            if hasattr(self, 'header_container'):
                self.header_container.height = self._get_cart_title_size()

    def create_column_titles(self):
        column_titles = BoxLayout(
            orientation='horizontal',
            size_hint_y=None,
            height=dp(30),
            padding=dp(5),
            spacing=dp(10)
        )
        
        titles = [
            ('Item', 0.4),
            ('Qty', 0.2),
            ('Price', 0.25),
            ('', 0.15)
        ]
        
        for text, size in titles:
            column_titles.add_widget(Label(
                text=text,
                size_hint_x=size,
                bold=True,
                color=(0.2, 0.2, 0.2, 1)
            ))
        return column_titles

    def create_items_area(self):
        # Create a ScrollView
        scroll_view = ScrollView(
            size_hint=(1, 1),
            do_scroll_x=False,
            bar_width=dp(10),
            scroll_type=['bars', 'content']
        )
        
        # Create the cart items layout
        self.cart_items_layout = BoxLayout(
            orientation='vertical',
            size_hint_y=None,
            spacing=dp(2)
        )
        self.cart_items_layout.bind(minimum_height=self.cart_items_layout.setter('height'))
        
        # Add cart items layout to scroll view
        scroll_view.add_widget(self.cart_items_layout)
        
        # Add scroll view to the cart upper section
        self.cart_upper.add_widget(scroll_view)

    def create_checkout_button(self):
        checkout_btn = Button(
            text='Checkout',
            size_hint=(1, None),
            height=dp(50),
            background_color=(0.2, 0.8, 0.2, 1),
            background_normal='',
            font_size='20sp',
            bold=True
        )
        
        with checkout_btn.canvas.before:
            Color(0.2, 0.8, 0.2, 1)
            self.checkout_rect = RoundedRectangle(
                pos=checkout_btn.pos,
                size=checkout_btn.size,
                radius=[25,]
            )
        checkout_btn.bind(pos=self.update_checkout_rect, size=self.update_checkout_rect)
        checkout_btn.bind(on_press=self.checkout)
        self.add_widget(checkout_btn)

    def update_clear_rect(self, instance, value):
        self.clear_rect.pos = instance.pos
        self.clear_rect.size = instance.size

    def update_checkout_rect(self, instance, value):
        self.checkout_rect.pos = instance.pos
        self.checkout_rect.size = instance.size

    def clear_cart(self, instance=None):  # Make instance parameter optional
        """Clear cart items and reset total"""
        self.cart_items = {}
        self.total = 0.0
        self.total_amount.text = '$0.00'  # Reset total amount display
        self.cart_items_layout.clear_widgets()
        self.update_display()

    def checkout(self, instance):
        if self.cart_items:
            popup = CheckoutPopup(
                total_amount=self.total,
                on_confirm=self.clear_cart  # Pass clear_cart directly
            )
            popup.open()

    def _process_checkout(self):
        """This method can be removed as we're using clear_cart directly"""
        pass

    def update_total(self):
        """Calculate total price from all items in cart"""
        self.total = sum(price * quantity for price, quantity in self.cart_items.values())
        # Update the display immediately
        if hasattr(self, 'total_amount'):
            self.total_amount.text = f'${self.total:.2f}'

    def update_display(self):
        self.cart_items_layout.clear_widgets()
        for product, (price, quantity) in self.cart_items.items():
            cart_item = CartItem(
                product=product,
                price=price,
                quantity=quantity,
                delete_callback=self.remove_item
            )
            self.cart_items_layout.add_widget(cart_item)

    def remove_item(self, product, price):
        """Decrease item quantity by 1 or remove if quantity reaches 0"""
        if product in self.cart_items:
            current_price, current_qty = self.cart_items[product]
            if current_qty > 1:
                # Decrease quantity by 1
                self.cart_items[product] = (price, current_qty - 1)
            else:
                # Remove item if quantity would be 0
                del self.cart_items[product]
            self.update_total()
            self.update_display()

    def update_cart_items(self):
        self.cart_items_layout.clear_widgets()
        for product, (price, quantity) in self.cart_items.items():
            cart_item = CartItem(
                product=product,  # Pass original text, CartItem will handle truncation
                price=price,
                quantity=quantity,
                delete_callback=self.remove_item
            )
            self.cart_items_layout.add_widget(cart_item)
