from kivy.uix.boxlayout import BoxLayout
from kivy.uix.scrollview import ScrollView
from kivy.metrics import dp
from kivymd.uix.label import MDLabel
from kivy.graphics import Color, RoundedRectangle
from kivymd.uix.button import MDIconButton, MDRaisedButton
from kivymd.uix.textfield import MDTextField
from data.product_data import products

class SalesHistoryView(BoxLayout):
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.orientation = 'vertical'
        self.padding = [15, 15, 15, 15]
        self.spacing = dp(10)
        self.is_edit_mode = False  # Add this flag to track mode
        self.is_update_mode = {}  # Add dictionary to track update mode for each row
        
        with self.canvas.before:
            Color(0.2, 0.2, 0.6, 1)
            self.bg = RoundedRectangle(pos=self.pos, size=self.size, radius=[15])
        self.bind(pos=self._update_rect, size=self._update_rect)

        # Add "Add Product" button at the top
        self.add_button = MDRaisedButton(
            text="Add Product",
            size_hint=(None, None),
            size=(dp(200), dp(60)),  # Match header size
            height=dp(40),
            pos_hint={'center_y': 0.5},  # Center vertically
            on_release=self.show_add_product_row
        )

        # Create header scroll view
        self.header_scroll = ScrollView(
            size_hint=(1, None),
            height=dp(60),  # Reduced from 80 to 60
            do_scroll_y=False,
            bar_width=0,  # Hide scrollbar for header
            scroll_type=['bars']  # Add scroll type
        )

        # Create header
        self.header = BoxLayout(
            size_hint=(None, None),
            height=dp(60),  # Reduced from 80 to 60
            width=dp(1200),
            spacing=dp(5),
            padding=[0, dp(10), dp(10), 0]  # Changed left padding to 0
        )
        
        with self.header.canvas.before:
            Color(0.2, 0.3, 0.9, 1)
            RoundedRectangle(pos=self.header.pos, size=self.header.size, radius=[5])
        self.header.bind(pos=self._update_header_bg)

        # Add header content
        headers = ['ID', 'Name', 'Price', 'Category', 'Stock']  # Removed 'Actions'
        self.header_labels = []  # Store references to header labels
        self.input_fields = []   # Store references to input fields
        
        # Create header labels and input fields
        for head in headers:  # No need to exclude 'Actions' anymore
            # Create header label
            label = MDLabel(
                text=head,
                size_hint=(None, None),
                size=(dp(200), dp(60)),  # Reduced height
                halign='center',
                valign='center',  # Added vertical centering
                bold=True,
                font_style='H6',  # Changed back to H6 from H5
                theme_text_color="Custom",
                text_color=(1, 1, 1, 1)
            )
            self.header_labels.append(label)
            self.header.add_widget(label)
            
            # Create input field (but don't add it yet)
            text_input = MDTextField(
                hint_text=head,
                size_hint=(None, None),
                size=(dp(200), dp(60)),  # Reduced height
                mode="rectangle",
                line_color_normal=(1, 1, 1, 1),
                text_color_normal=(1, 1, 1, 1),
                hint_text_color_normal=(0.8, 0.8, 0.8, 1)
            )
            self.input_fields.append(text_input)

        # Add the add_button to the header instead of Actions label
        self.header.add_widget(self.add_button)

        # Create action buttons container
        self.action_buttons = BoxLayout(
            size_hint=(None, None),
            size=(dp(200), dp(60)),  # Reduced height
            spacing=dp(10),
            padding=[dp(5), dp(10), 0, dp(10)]  # Reduced vertical padding
        )
        
        # Create save/cancel buttons
        self.save_btn = MDRaisedButton(  # Changed to instance variable
            text=self._get_save_button_text(),  # Use helper method
            theme_text_color="Custom",
            text_color=(1, 1, 1, 1),
            md_bg_color=(0, 0.7, 0, 1),
            on_release=self.save_new_product
        )
        
        cancel_btn = MDRaisedButton(  # Changed from MDIconButton to MDRaisedButton
            text="Cancel",            # Use text instead of icon
            theme_text_color="Custom",
            text_color=(1, 1, 1, 1),
            md_bg_color=(0.8, 0, 0, 1),
            on_release=self.cancel_add_product  # Changed from clear_input_fields to cancel_add_product
        )
        
        self.action_buttons.add_widget(self.save_btn)
        self.action_buttons.add_widget(cancel_btn)

        self.header_scroll.add_widget(self.header)
        self.add_widget(self.header_scroll)

        # Create content scroll view
        self.scroll = ScrollView(
            do_scroll_x=True,
            do_scroll_y=True,
            effect_cls='ScrollEffect',
            bar_width=dp(10),
            scroll_type=['bars']  # Add scroll type
        )
        
        # Bind both scrollviews to sync in both directions
        self.header_scroll.bind(scroll_x=self._sync_scroll)
        self.scroll.bind(scroll_x=self._sync_scroll)
        
        # Content container
        self.container = BoxLayout(
            orientation='vertical',
            size_hint=(None, None),
            spacing=dp(5),
            width=dp(1200)  # Match header width
        )
        self.container.bind(
            minimum_height=self.container.setter('height'),
            minimum_width=self.container.setter('width')
        )
        
        self.scroll.add_widget(self.container)
        self.add_widget(self.scroll)

        # Initialize with product data
        self.update_products(products)

    def _sync_scroll(self, instance, value):
        """Synchronize scrolling between header and content"""
        if instance == self.header_scroll:
            # Header was scrolled, update content
            if self.scroll.scroll_x != value:
                self.scroll.scroll_x = value
        else:
            # Content was scrolled, update header
            if self.header_scroll.scroll_x != value:
                self.header_scroll.scroll_x = value

    def update_products(self, product_data):
        self.container.clear_widgets()
        self.is_update_mode = {}  # Reset update mode tracking
        
        for prod in product_data:
            # Create row with background
            row = BoxLayout(
                size_hint=(None, None),
                height=dp(40),
                width=dp(1200),
                spacing=dp(5)
            )
            
            # Create canvas instructions
            with row.canvas.before:
                color = Color(0.2, 0.2, 0.4, 0.1)  # Start with slight base color
                bg_rect = RoundedRectangle(
                    pos=row.pos,
                    size=row.size,
                    radius=[5,]
                )

            # Bind position and size updates
            def update_bg(instance, value, bg=bg_rect):
                bg.pos = instance.pos
                bg.size = instance.size
            
            row.bind(pos=update_bg, size=update_bg)
            
            fields = [
                str(prod[0]),
                str(prod[1]),
                f"${prod[2]:.2f}",
                str(prod[3]),
                str(prod[4])
            ]
            
            # Add text fields with white color
            for field in fields:
                label = MDLabel(
                    text=field,
                    size_hint=(None, None),
                    size=(dp(200), dp(40)),
                    halign='center',
                    theme_text_color="Custom",
                    text_color=(1, 1, 1, 1)  # White text
                )
                row.add_widget(label)

            # Add action buttons container
            actions = BoxLayout(
                size_hint=(None, None),
                size=(dp(200), dp(40)),
                spacing=dp(10),
                padding=[dp(20), 0, 0, 0]
            )
            
            # Update button with new switching functionality
            update_btn = MDIconButton(
                icon="pencil",
                theme_text_color="Custom",
                text_color=(0, 0.7, 0, 1),
                on_release=lambda x, id=prod[0]: self.toggle_update_mode(id, x)
            )
            
            # Store reference to update button
            self.is_update_mode[prod[0]] = {
                "button": update_btn,
                "is_editing": False,
                "row": row,
                "bg_color": color,  # Store Color instruction
                "bg_rect": bg_rect  # Store Rectangle instruction
            }
            
            # Delete button
            delete_btn = MDIconButton(
                icon="delete",
                theme_text_color="Custom",
                text_color=(0.8, 0, 0, 1),
                on_release=lambda x, id=prod[0]: self.on_delete(id)
            )
            
            actions.add_widget(update_btn)
            actions.add_widget(delete_btn)
            row.add_widget(actions)
            
            self.container.add_widget(row)

    def show_add_product_row(self, *args):
        if not self.is_edit_mode:
            # Switch to edit mode
            self.is_edit_mode = True
            self.add_button.text = "Adding New Product"
            
            # Replace header labels with input fields
            for i, (label, field) in enumerate(zip(self.header_labels, self.input_fields)):
                if label in self.header.children:
                    self.header.remove_widget(label)
                if field.parent:
                    field.parent.remove_widget(field)
                self.header.add_widget(field, index=len(self.header_labels) - i - 1)
            
            # Replace add_button with action buttons
            if self.add_button in self.header.children:
                self.header.remove_widget(self.add_button)
            if self.action_buttons.parent:
                self.action_buttons.parent.remove_widget(self.action_buttons)
            self.header.add_widget(self.action_buttons)
        else:
            # Switch back to view mode
            self.cancel_add_product()

    def clear_input_fields(self, *args):
        """Clear all input fields without changing the view"""
        for field in self.input_fields:
            field.text = ''
            if hasattr(field, 'error'):
                field.error = False
            field.helper_text = ''

    def save_new_product(self, *args):
        # Get values from input fields
        values = [field.text for field in self.input_fields]
        try:
            # Convert price and stock to appropriate types
            values[2] = float(values[2])  # Price to float
            values[4] = int(values[4])    # Stock to int
            
            # Add new product to data
            new_product = tuple(values)
            products.append(new_product)
            
            # Refresh the view
            self.update_products(products)
            
        except ValueError:
            print("Invalid input values")
        
        # Reset the header view after saving
        self.clear_input_fields()  # Use new method to clear fields after saving
        self.cancel_add_product()  # Still need this to restore view

    def cancel_add_product(self, *args):
        # Reset all update buttons to pencil and their states
        for pid, data in self.is_update_mode.items():
            data["button"].icon = "pencil"
            data["is_editing"] = False
            data["bg_color"].rgba = (0.2, 0.2, 0.4, 0.1)

        # Clear all input fields and restore labels
        self.is_edit_mode = False
        self.add_button.text = "Add Product"
        
        # Clear and restore labels
        for i, (label, field) in enumerate(zip(self.header_labels, self.input_fields)):
            # Clear field
            field.text = ''
            # Remove field if present
            if field in self.header.children:
                self.header.remove_widget(field)
            # Restore label
            if label.parent:
                label.parent.remove_widget(label)
            self.header.add_widget(label, index=len(self.header_labels) - i - 1)
        
        # Restore add_button
        if self.action_buttons in self.header.children:
            self.header.remove_widget(self.action_buttons)
        if self.add_button.parent:
            self.add_button.parent.remove_widget(self.add_button)
        self.header.add_widget(self.add_button)

    def on_update(self, product_id):
        print(f"Update product: {product_id}")
        # Add your update logic here

    def on_delete(self, product_id):
        print(f"Delete product: {product_id}")
        # Add your delete logic here

    def toggle_update_mode(self, product_id, button_instance):
        """Toggle between edit and save modes for update button"""
        if not self.is_update_mode[product_id]["is_editing"]:
            # Reset all other rows to default state first
            for pid, data in self.is_update_mode.items():
                if pid != product_id and data["is_editing"]:
                    # Reset other row that was being edited
                    data["button"].icon = "pencil"
                    data["is_editing"] = False
                    data["bg_color"].rgba = (0.2, 0.2, 0.4, 0.1)
                    print(f"Canceling edit for product {pid}")
            
            # Switch current row to edit mode
            button_instance.icon = "alert-circle"
            self.is_update_mode[product_id]["is_editing"] = True
            # Change row background to bright color
            self.is_update_mode[product_id]["bg_color"].rgba = (0.4, 0.6, 0.8, 0.5)
            print(f"Editing product {product_id}")
            
            # Always trigger show_add_product_row when entering warning mode
            if not self.is_edit_mode:  # Only if not already in edit mode
                self.show_add_product_row()
            
            # Update save button text when entering warning mode
            self.save_btn.text = self._get_save_button_text()
            
        else:
            # Switch back to normal mode
            button_instance.icon = "pencil"
            self.is_update_mode[product_id]["is_editing"] = False
            # Reset row background color
            self.is_update_mode[product_id]["bg_color"].rgba = (0.2, 0.2, 0.4, 0.1)
            print(f"Saving changes for product {product_id}")
            self.save_product_updates(product_id)
            
            # Switch back to view mode if no other rows are in edit mode
            if self.is_edit_mode and not any(data["is_editing"] for data in self.is_update_mode.values()):
                self.show_add_product_row()
            
            # Update save button text when exiting warning mode
            self.save_btn.text = self._get_save_button_text()

    def save_product_updates(self, product_id):
        """Handle saving the updated product data"""
        # Add your save logic here
        print(f"Saving updates for product {product_id}")

    def _get_save_button_text(self):
        """Helper method to determine save button text"""
        return "Update" if any(data["is_editing"] for data in self.is_update_mode.values()) else "Add"

    def _update_rect(self, instance, value):
        self.bg.pos = instance.pos
        self.bg.size = instance.size

    def _update_header_bg(self, *args):
        """Update header background position"""
        self.header.canvas.before.clear()
        with self.header.canvas.before:
            Color(0.2, 0.3, 0.9, 1)
            RoundedRectangle(pos=self.header.pos, size=self.header.size, radius=[5])
