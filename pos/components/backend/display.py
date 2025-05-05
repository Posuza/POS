from kivy.uix.boxlayout import BoxLayout
from kivy.metrics import dp
from kivy.uix.label import Label
from kivy.uix.widget import Widget
from kivy.graphics import Color, Rectangle, RoundedRectangle
from kivymd.uix.button import MDRaisedButton, MDIconButton
from kivymd.uix.label import MDLabel  # Add this import
from kivymd.app import MDApp
from kivy.core.window import Window  # Add this import
# Update view imports to use new path
from dashboradBodyViews.dashboard_view import  DashboardView
from dashboradBodyViews.sale_view import SalesHistoryView
from dashboradBodyViews.help_view import HelpView
from dashboradBodyViews.products_view import ProductsView
from dashboradBodyViews.settings_view import GeneralSettingsView
from dashboradBodyViews.accounts_view import StaffAccountsView
from dashboradBodyViews.default_view import DefaultView

class Display(BoxLayout):
    def __init__(self, add_to_cart_callback, **kwargs):
        super().__init__(**kwargs)
        self.orientation = 'vertical'
        self.size_hint_x = 1
        # self.padding = [0, 0, 0, 0]  # Add padding to main container
        
        # Header Section with three parts (left, center, right)
        self.header = BoxLayout(
            orientation='horizontal',
            size_hint_y=None,
            height=dp(60),
            spacing=dp(10)
        )
        
        # Add background to header
        # with self.header.canvas.before:
        #     Color(0.9, 0.9, 0.9, 1)  # Light gray
        #     self.header_bg = Rectangle(pos=self.header.pos, size=self.header.size)
        # self.header.bind(pos=self._update_header_bg, size=self._update_header_bg)
        
        # Left section with admin name
        left_container = BoxLayout(
            orientation='horizontal',
            size_hint_x=0.25,  # Changed to 25%

        )
        # Add background to left container
        # with left_container.canvas.before:
        #     Color(0.95, 0.95, 1, 1)  # Light blue
        #     self.left_bg = Rectangle(pos=left_container.pos, size=left_container.size)
        # left_container.bind(pos=self._update_left_bg, size=self._update_left_bg)
        
        self.admin_label = Label(
            text='',
            font_size='16sp',  # Slightly smaller font
            color=(0.2, 0.2, 0.2, 1),
            bold=True,
            size_hint_x=1,
            halign='left',
            valign='middle'
        )
        left_container.add_widget(self.admin_label)
        
        # Center section with title
        center_container = BoxLayout(
            orientation='horizontal',
            size_hint_x=0.25,  # Changed to 25%

        )
        # Add background to center container
        # with center_container.canvas.before:
        #     Color(1, 0.95, 0.95, 1)  # Light pink
        #     self.center_bg = Rectangle(pos=center_container.pos, size=center_container.size)
        # center_container.bind(pos=self._update_center_bg, size=self._update_center_bg)
        
        # Add both label and icon (icon initially hidden)
        self.header_label = Label(
            text='Dashboard',
            font_size='20sp',
            color=(0.2, 0.2, 0.2, 1),
            bold=True,
            size_hint_x=1,
            halign='center',
            opacity=1
        )
        
        self.header_icon = MDIconButton(
            icon="view-dashboard",
            size_hint=(None, None),
            size=(dp(40), dp(40)),
            pos_hint={'center_x': 0.5, 'center_y': 0.5},
            theme_text_color="Custom",
            text_color=(0.2, 0.2, 0.2, 1),
            opacity=0
        )
        
        center_container.add_widget(self.header_label)
        center_container.add_widget(self.header_icon)
        
        # Bind width changes to update visibility
        center_container.bind(width=self._update_header_visibility)
        
        # Right section with buttons
        right_container = BoxLayout(
            orientation='horizontal',
            size_hint_x=0.50,  # Changed to 50%
            spacing=dp(0),
            padding=[0, 0, dp(20), 0]
        )
        # Add background to right container
        # with right_container.canvas.before:
        #     Color(0.95, 1, 0.95, 1)  # Light green
        #     self.right_bg = Rectangle(pos=right_container.pos, size=right_container.size)
        # right_container.bind(pos=self._update_right_bg, size=self._update_right_bg)
        
        # Only keep notification button
        self.notification_btn = MDIconButton(
            icon="bell",
            size_hint=(None, None),
            size=(dp(40), dp(40)),
            pos_hint={'center_y': 0.5},
            theme_text_color="Custom",
            text_color=(0.2, 0.4, 0.8, 1)
        )
        
        # Create logout button with background
        self.logout_item = BoxLayout(
            orientation='horizontal',
            size_hint=(None, None),
            size=(dp(120), dp(48)),
            spacing=dp(0),
            pos_hint={'center_y': 0.5},
            padding=[dp(0), dp(0), dp(10), dp(0)]
        )

        # Add background and rounded corners
        with self.logout_item.canvas.before:
            Color(0.2, 0.4, 0.8, 1)  # Blue background
            self.logout_bg = RoundedRectangle(
                pos=self.logout_item.pos,
                size=self.logout_item.size,
                radius=[dp(24)]
            )
        self.logout_item.bind(pos=self._update_logout_bg, size=self._update_logout_bg)

        # Logout icon
        self.logout_icon = MDIconButton(
            icon="exit-to-app",
            size_hint=(None, None),
            size=(dp(40), dp(40)),
            theme_text_color="Custom",
            text_color=(1, 1, 1, 1),
            pos_hint={'center_y': 0.5},
            padding=dp(0)
        )
        
        # Logout label
        self.logout_label = MDLabel(
            text="Logout",
            theme_text_color="Custom",
            text_color=(1, 1, 1, 1),
            size_hint=(None, None),
            size=(dp(70), dp(48)),
            halign='left',
            valign='center',
            padding=[dp(0), 0]
        )

        # Add widgets to logout container
        self.logout_item.add_widget(self.logout_icon)
        self.logout_item.add_widget(self.logout_label)

        # Bind the entire layout to logout
        self.logout_item.bind(on_touch_down=lambda x, y: self.logout(None) if self.logout_item.collide_point(*y.pos) else None)

        # Button container
        button_container = BoxLayout(
            orientation='horizontal',
            size_hint=(None, None),
            size=(dp(170), dp(48)),
            spacing=dp(10),
            pos_hint={'right': 1, 'center_y': 0.5}
        )
        
        # Add buttons to container
        button_container.add_widget(self.notification_btn)
        button_container.add_widget(self.logout_item)

        # Add to right container
        right_container.add_widget(Widget(size_hint_x=1))
        right_container.add_widget(button_container)
        
        # Add all containers to header
        self.header.add_widget(left_container)
        self.header.add_widget(center_container)
        self.header.add_widget(right_container)
        
        # Body Section
        self.body = BoxLayout(
            orientation='vertical',
            size_hint_y=0.9,
            padding=[0, 0, dp(10), dp(10)]  # [left, top, right, bottom] padding
        )
        
        # Add background to body
        with self.body.canvas.before:
            Color(0.95, 0.95, 0.95, 1)
            self.body_bg = Rectangle(pos=self.body.pos, size=self.body.size)
        self.body.bind(pos=self._update_body_bg, size=self._update_body_bg)
        
        # Initialize with default DashboardView
        self.current_view = DashboardView()
        self.body.add_widget(self.current_view)
        
        # Add sections to main layout
        self.add_widget(self.header)
        self.add_widget(self.body)
        
        # Store callback
        self.add_to_cart_callback = add_to_cart_callback

    def _update_header_bg(self, instance, value):
        self.header_bg.pos = instance.pos
        self.header_bg.size = instance.size
    
    def _update_body_bg(self, instance, value):
        self.body_bg.pos = instance.pos
        self.body_bg.size = instance.size
    
    def _update_left_bg(self, instance, value):
        self.left_bg.pos = instance.pos
        self.left_bg.size = instance.size
    
    def _update_center_bg(self, instance, value):
        self.center_bg.pos = instance.pos
        self.center_bg.size = instance.size
    
    def _update_right_bg(self, instance, value):
        self.right_bg.pos = instance.pos
        self.right_bg.size = instance.size
    
    def _update_logout_bg(self, instance, value):
        """Update logout button background"""
        self.logout_bg.pos = instance.pos
        self.logout_bg.size = instance.size

    def set_username(self, username):
        """Update admin name only"""
        self.admin_label.text = f'{username}'
    
    def handle_view_switch(self, view_name, icon):
        self.body.clear_widgets()
        """Update both header label and icon when view changes"""
        print(f"Switching to view: {view_name}")  # Add debug print
        self.header_label.text = view_name
        self.header_icon.icon = icon
        
        # Clear current view
        
        # Create and add new view with error handling
        try:
            print(view_name)
            # Dynamically get the view class based on the view_name string
            view_class = globals().get(view_name)
            if view_class:
                # Instantiate the view class
                self.current_view = view_class()
                self.body.add_widget(self.current_view)
                print(f"Successfully added {view_name} view")  # Add debug print
            else:
                raise ValueError(f"View class for '{view_name}' not found")
        except Exception as e:
            print(f"Error creating view: {e}")  # Add debug print
            # Fallback to dashboard if there's an error
            self.current_view = DefaultView(view_name)  # Assuming ProductView is a fallback view
            self.body.add_widget(self.current_view)

  
    
    def _update_header_visibility(self, instance, width):
        """Switch between icon and text based on width and update text length"""
        if width < dp(150):  # Very small screens
            self.header_label.opacity = 0
            self.header_icon.opacity = 1
            self.logout_label.text = ""  # Hide logout text
        elif width < dp(300):  # Small screens
            self.header_label.opacity = 1
            self.header_icon.opacity = 0
            current_text = self.header_label.text
            self.header_label.text = current_text[:6] + "..." if len(current_text) > 6 else current_text
            self.logout_label.text = "Logout"  # Short version
        elif width < dp(400):  # Medium screens
            self.header_label.opacity = 1
            self.header_icon.opacity = 0
            current_text = self.header_label.text
            self.header_label.text = current_text[:10] + "..." if len(current_text) > 10 else current_text
            self.logout_label.text = "Logout"  # Full version
        else:  # Large screens
            self.header_label.opacity = 1
            self.header_icon.opacity = 0
            self.header_label.text = self._get_full_header_text()  # Original text
            self.logout_label.text = "Logout"  # Full version

    def _get_full_header_text(self):
        """Return the full text based on current view"""
        return self.current_view.__class__.__name__.replace('View', '')

    def logout(self, instance):
        """Handle logout and reset labels"""
        app = MDApp.get_running_app()
        self.header_label.text = 'Dashboard'  # Reset to default view
        self.admin_label.text = ''  # Reset admin label
        app.sm.transition.direction = 'right'
        app.sm.current = 'login'
