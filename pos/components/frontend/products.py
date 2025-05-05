from kivymd.app import MDApp  # Make sure this import is at the top
from kivy.uix.boxlayout import BoxLayout
from kivy.uix.gridlayout import GridLayout
from kivy.uix.button import Button
from kivy.uix.label import Label
from kivy.uix.textinput import TextInput
from kivy.uix.scrollview import ScrollView  # Add this import
from kivy.metrics import dp
from kivy.graphics import Color, Rectangle, RoundedRectangle  # Add RoundedRectangle
from kivymd.uix.button import MDIconButton
from kivymd.uix.label import MDLabel
from kivymd.uix.button import MDRaisedButton
from data.product_data import products
from data.categories_data import categories  # Add this import
from components.frontend.productCard import ProductCard  # Update this import
from kivy.core.window import Window

class ProductSection(BoxLayout):
    def __init__(self, add_to_cart_callback, **kwargs):
        super().__init__(**kwargs)
        self.orientation = 'vertical'
        self.spacing = dp(2)  # Reduce spacing between rows
        self.padding = [dp(2), dp(2), dp(2), dp(2)]  # Minimal padding
        self.add_to_cart_callback = add_to_cart_callback
        self.products = products
        self.current_category = 'All'
        
        # Add gray background
        with self.canvas.before:
            Color(0.95, 0.95, 0.95, 1)
            self.rect = Rectangle(pos=self.pos, size=self.size)
        self.bind(size=self._update_rect, pos=self._update_rect)
        
        # Header with username and logout
        header = BoxLayout(
            orientation='horizontal',
            size_hint_y=None,
            height=dp(50),
            spacing=dp(10)
        )
        
        self.username_label = MDLabel(
            text="Products",
            font_style="H5",
            size_hint_x=0.9
        )
        
        self.logout_button = MDRaisedButton(
            text="Logout",
            size_hint=(None, None),
            height=dp(40),
            on_release=self.logout
        )
        
        header.add_widget(self.username_label)
        header.add_widget(self.logout_button)
        
        # Add header to main layout
        self.add_widget(header)
        
        # Title and Search Section - responsive orientation
        self.header_section = BoxLayout(
            orientation='horizontal',
            size_hint_y=None,
            height=dp(50),
            spacing=dp(10),
            padding=[dp(10), dp(20), dp(10), 0]
        )
        
        # Search Container - full width
        search_container = BoxLayout(
            orientation='horizontal',
            size_hint_x=1,  # Take full width
            height=dp(30),
            pos_hint={'center_y': 0.4}
        )
        
        # Add rounded background to search container
        with search_container.canvas.before:
            Color(1, 1, 1, 1)
            self.search_rect = RoundedRectangle(
                pos=search_container.pos,
                size=search_container.size,
                radius=[dp(15),]
            )
        search_container.bind(pos=self._update_search_rect, size=self._update_search_rect)
        
        # Updated Search Bar with binding
        self.search_bar = TextInput(
            size_hint_x=0.9,
            size_hint_y=None,
            height=dp(30),
            hint_text='Search by product name or serial number...',
            multiline=False,
            background_color=(1, 1, 1, 0),
            foreground_color=(0.2, 0.2, 0.2, 1),
            cursor_color=(0.2, 0.2, 0.2, 1),
            font_size='16sp',
            padding=[dp(15), dp(5)],
            pos_hint={'center_y': 0.5}
        )
        self.search_bar.bind(text=self.on_search_text)
        
        # Search Icon
        search_btn = MDIconButton(
            icon="magnify",
            pos_hint={"center_y": .5},
            theme_text_color="Custom",
            text_color=(1, 1, 1, 1),
            md_bg_color=(0.3, 0.5, 0.7, 1),
            size_hint=(0.1, 1.1),
            size=(dp(40), dp(40))
        )
        
        search_container.add_widget(self.search_bar)
        search_container.add_widget(search_btn)
        
        # Add search container directly to header section
        self.header_section.add_widget(search_container)
        
        # Add header section to main layout
        self.add_widget(self.header_section)
        
        # Categories Section with ScrollView
        categories_container = BoxLayout(
            orientation='vertical',
            size_hint_y=None,
            height=dp(70),
            padding=[0, 20, 0, 20]
        )
        
        # Horizontal ScrollView for categories
        categories_scroll = ScrollView(
            size_hint=(1, None),
            height=dp(50),
            do_scroll_y=False,  # Disable vertical scroll
            do_scroll_x=True,   # Enable horizontal scroll
            bar_width=0,        # Hide scrollbar
            scroll_type=['bars', 'content']
        )
        
        # Categories Layout
        self.categories_section = BoxLayout(
            orientation='horizontal',
            size_hint_x=None,   # Required for horizontal scroll
            height=dp(40),
            spacing=dp(10),
            padding=[dp(10), 0]  # Add horizontal padding
        )
        
        # Bind width to minimum width for scrolling
        self.categories_section.bind(minimum_width=self.categories_section.setter('width'))
        
        # Use imported categories with quantities
        for category_name, qty in categories:
            cat_container = BoxLayout(
                orientation='vertical',
                size_hint=(None, None),
                width=dp(100),
                height=dp(38),
                padding=[0, 0, 0, 20]
            )
            
            # Add rounded background for category button
            with cat_container.canvas.before:
                Color(0.3, 0.5, 0.7, 1)
                cat_rect = RoundedRectangle(
                    pos=cat_container.pos,
                    size=cat_container.size,
                    radius=[dp(18),]
                )
            cat_container.bind(
                pos=lambda obj, val, rect=cat_rect: setattr(rect, 'pos', val),
                size=lambda obj, val, rect=cat_rect: setattr(rect, 'size', val)
            )

            # Category button
            button = Button(
                text=category_name,
                background_color=(0, 0, 0, 0),
                color=(1, 1, 1, 1),
                font_size='16sp',
                bold=True,
                size_hint=(1, 1)
            )
            button.bind(on_press=lambda x, cat=category_name: self.filter_by_category(cat))

            # Quantity badge - notification style
            qty_container = BoxLayout(
                size_hint=(None, None),
                size=(dp(22), dp(22)),  # Smaller size for notification style
                pos_hint={'right': 1.02, 'top': 0}  # Position above button
            )
            
            # Add badge background - notification style with consistent color
            with qty_container.canvas.before:
                Color(0.2, 0.8, 0.2, 1) # Notification red color
                qty_rect = RoundedRectangle(
                    pos=qty_container.pos,
                    size=qty_container.size,
                    radius=[dp(20),]  # Full circle
                )
            qty_container.bind(
                pos=lambda obj, val, rect=qty_rect: setattr(rect, 'pos', val),
                size=lambda obj, val, rect=qty_rect: setattr(rect, 'size', val)
            )
            
            qty_label = Label(
                text=str(qty),
                font_size='11sp',  # Smaller font for notification style
                bold=True,
                color=(0.2, 0.2, 0.2, 1),  # White text
                size_hint=(None, None),
                size=qty_container.size
            )
            
            qty_container.add_widget(qty_label)
            cat_container.add_widget(qty_container)
            cat_container.add_widget(button)
            
            self.categories_section.add_widget(cat_container)
        
        # Add categories to scroll view and container
        categories_scroll.add_widget(self.categories_section)
        categories_container.add_widget(categories_scroll)
        self.add_widget(categories_container)
        
        # Products Grid with ScrollView and Border
        products_container = BoxLayout(
            orientation='vertical',
            padding=[dp(1), dp(1)]  # Border thickness
        )
        
        # Add border with rounded corners
        with products_container.canvas.before:
            Color(0.8, 0.8, 0.8, 1)  # Border color (light gray)
            self.border_rect = RoundedRectangle(
                pos=products_container.pos,
                size=products_container.size,
                radius=[dp(10),]
            )
        products_container.bind(pos=self._update_border, size=self._update_border)

        scroll_view = ScrollView(
            do_scroll_x=False,
            bar_width=dp(7),
            scroll_type=['bars', 'content']
        )
        
        # Set up grid layout with tighter spacing
        self.grid = GridLayout(
            cols=4,  # Adjust number of columns as needed
            spacing=(dp(2), dp(2)),  # Reduce horizontal and vertical spacing
            size_hint_y=None,
            padding=[dp(2), dp(2), dp(2), dp(2)]  # Minimal padding
        )
        self.grid.bind(minimum_height=self.grid.setter('height'))
        
        # Bind window size to update grid
        Window.bind(size=self._on_window_resize)
        
        # Add grid to scroll view and scroll view to border container
        scroll_view.add_widget(self.grid)
        products_container.add_widget(scroll_view)
        self.add_widget(products_container)
        
        self.create_products()

    def _update_rect(self, instance, value):
        self.rect.pos = instance.pos
        self.rect.size = instance.size

    def _update_search_rect(self, instance, value):
        self.search_rect.pos = instance.pos
        self.search_rect.size = instance.size

    def _update_border(self, instance, value):
        self.border_rect.pos = instance.pos
        self.border_rect.size = instance.size

    def filter_by_category(self, category):
        self.current_category = category
        self.grid.clear_widgets()
        self.create_products()

    def _get_grid_cols(self):
        """Calculate number of columns based on window width"""
        width = Window.width
        if width <= 800:  # Phone
            return 4
        elif width <= 1024:  # Tablet
            return 5
        else:  # Desktop
            return 5

    def _calculate_responsive_layout(self):
        """Calculate responsive layout parameters based on screen size"""
        width = Window.width
        if width <= 800:  # Phone
            cols = 4
            min_card_width = dp(70)  # Even smaller for phone
            spacing = dp(4)
            padding = dp(4)
            cat_width = dp(70)
            cat_height = dp(25)
            cat_font_size = '11sp'
        elif width <= 1024:  # Tablet
            cols = 5
            min_card_width = dp(90)  # Smaller for tablet
            spacing = dp(6)
            padding = dp(8)
            cat_width = dp(85)
            cat_height = dp(32)
            cat_font_size = '13sp'
        else:  # Desktop
            cols = 5
            min_card_width = dp(160)
            spacing = dp(15)
            padding = dp(15)
            cat_width = dp(100)
            cat_height = dp(38)
            cat_font_size = '16sp'

        # Calculate actual card width
        available_width = self.width - (2 * padding) - ((cols - 1) * spacing)
        card_width = max(min_card_width, available_width / cols)
        
        return {
            'cols': cols,
            'card_width': card_width,
            'spacing': spacing,
            'padding': padding,
            'cat_width': cat_width,
            'cat_height': cat_height,
            'cat_font_size': cat_font_size,
            'badge_size': dp(16) if width <= 800 else dp(18) if width <= 1024 else dp(22),
            'badge_font': '9sp' if width <= 800 else '10sp' if width <= 1024 else '11sp'
        }

    def _update_category_sizes(self):
        """Update category sizes based on screen width"""
        layout = self._calculate_responsive_layout()
        for child in self.categories_section.children:
            child.width = layout['cat_width']
            child.height = layout['cat_height']
            # Update category button font size
            button = child.children[0]
            button.font_size = layout['cat_font_size']
            # Update badge size
            badge = child.children[1]
            badge.size = (layout['badge_size'], layout['badge_size'])
            badge.children[0].font_size = layout['badge_font']

    def _update_grid_layout(self):
        """Update grid layout based on current screen size"""
        layout = self._calculate_responsive_layout()
        self.grid.cols = layout['cols']
        self.grid.spacing = (layout['spacing'], layout['spacing'])
        self.grid.padding = [layout['padding']]
        return layout['card_width']

    def _on_window_resize(self, instance, value):
        """Update layout when window resizes"""
        width = value[0]
        # Update search container only
        search_container = self.header_section.children[0]
        search_container.size_hint_x = 1  # Always full width
        
        # Existing resize handling
        self._update_grid_layout()
        self._update_category_sizes()
        if hasattr(self, 'search_bar') and self.search_bar.text:
            self.on_search_text(None, self.search_bar.text)
        else:
            self.create_products()

    def _calculate_card_width(self):
        """Calculate responsive card width based on screen size"""
        width = Window.width
        cols = self._get_grid_cols()
        spacing_total = (cols - 1) * self.grid.spacing[0]
        padding_total = self.grid.padding[0] * 2
        available_width = (self.width) - spacing_total - padding_total - dp(30)
        
        # Base card width on screen size
        if width <= 800:  # Phone
            min_width = dp(120)
        elif width <= 1024:  # Tablet
            min_width = dp(140)
        else:  # Desktop
            min_width = dp(160)
            
        return max(min_width, available_width / cols)

    def create_products(self):
        self.grid.clear_widgets()
        filtered_products = [
            (serial_no, name, price, qty) 
            for serial_no, name, price, cat, qty in self.products 
            if self.current_category == 'All' or cat == self.current_category
        ]
        
        # Get responsive card width
        card_width = self._update_grid_layout()
        
        # Create product cards with proper callback
        for serial_no, name, price, qty in filtered_products:
            product_card = ProductCard(
                serial=serial_no,
                product=name,
                price=price,
                quantity=qty,
                callback=self.add_to_cart_callback,  # Pass the callback function
                size_hint_x=None,
                width=card_width
            )
            self.grid.add_widget(product_card)

    def on_search_text(self, instance, value):
        """Filter products based on search text"""
        self.grid.clear_widgets()
        search_term = value.lower().strip()
        
        if not search_term:  # If search is empty, show all products for current category
            self.create_products()
            return
        
        # Filter products based on search term and current category
        filtered_products = [
            (serial_no, name, price, qty) 
            for serial_no, name, price, cat, qty in self.products 
            if (self.current_category == 'All' or cat == self.current_category) and
               (search_term in name.lower() or search_term in str(serial_no).lower())
        ]
        
        # Get responsive card width
        card_width = self._update_grid_layout()
        
        # Create product cards for filtered results
        for serial_no, name, price, qty in filtered_products:
            product_card = ProductCard(
                serial=serial_no,
                product=name,
                price=price,
                quantity=qty,
                callback=self.add_to_cart_callback,
                size_hint_x=None,
                width=card_width
            )
            self.grid.add_widget(product_card)

    def set_username(self, username):
        """Update the header with username"""
        self.username_label.text = f"Welcome, {username}"
    
    def logout(self, instance):
        """Handle logout action with proper cleanup"""
        app = MDApp.get_running_app()
        
        # Clear any stored data or state
        self.username_label.text = "Products"
        if hasattr(self, 'search_bar'):
            self.search_bar.text = ""
        
        # Reset any filters
        self.current_category = 'All'
        
        # Clear grid
        self.grid.clear_widgets()
        
        # Create fresh products view
        self.create_products()
        
        # Switch to login screen
        app.sm.transition.direction = 'right'  # Add transition
        app.sm.current = 'login'
