from kivy.uix.boxlayout import BoxLayout
from kivy.metrics import dp
from kivy.animation import Animation
from kivy.uix.behaviors import ButtonBehavior
from kivymd.uix.label import MDIcon, MDLabel
from kivymd.uix.list import OneLineIconListItem, IconLeftWidget, TwoLineIconListItem, IconRightWidget
from kivy.properties import ObjectProperty
from kivy.graphics import Color, Rectangle, Line
from kivy.core.window import Window
from kivymd.uix.behaviors import HoverBehavior
from kivymd.uix.boxlayout import MDBoxLayout

current_expanded_panel = None
global_active_button = None
active_button = None


class ClickableIconButton(ButtonBehavior, HoverBehavior, MDBoxLayout):
    def __init__(self, icon_text, title, header, view_name, active_header, switch_view_callback, reset_global_bg, **kwargs):
        super().__init__(**kwargs)
        self.orientation = 'horizontal'
        self.size_hint_y = None
        self.height = dp(50)
        self.iconToggled = False
        self.padding = [dp(15), dp(10), dp(15), dp(10)]
        self.spacing = dp(10)
        self.header = header
        self.active_header = active_header
        self.original_text = title
        self.view_name = view_name
        self.press_active = False
        self.switch_view_callback = switch_view_callback
        self.reset_global_bg = reset_global_bg
        self.icon_text = icon_text


        self.default_bg_color = (0.95, 0.95, 0.95, 1)  # Light gray (unchanged)
        self.active_bg_color = (0.4, 0.7, 0.95, 1)  # Lighter soft blue  # Soft blue
        self.hover_bg_color = (0.9, 0.9, 0.9, 1)  # Slightly darker grayarker gray

        if not self.header and not self.active_header:
            self.md_bg_color = self.default_bg_color   # Light gray background

        # Left side container
        left_box = BoxLayout(
            orientation='horizontal',
            size_hint_x=1,
            spacing=dp(10)
        )
      
        self.icon = MDIcon(
            icon=self.icon_text,
            theme_text_color="Primary",
            size_hint=(None, None),
            size=(dp(24), dp(24)),
            pos_hint={'center_y': 0.5}
        )
        
        self.title_item = MDLabel(
            text=self._get_truncated_text(title),
            theme_text_color="Primary",
            size_hint_x=1,
            pos_hint={'center_y': 0.5},
            shorten=True,  # Enable text shortening
            shorten_from='right',  # Shorten from right side
            text_size=(None, None),
            # padding_x=(dp(10), 0)
        )
        
        left_box.add_widget(self.icon)
        left_box.add_widget(self.title_item)
        self.add_widget(left_box)
       

        if self.header and active_header:
            if active_header:
                new_icon = "plus-circle" 
            else:
                new_icon = "minus-circle"
            self.arrow_icon = MDIcon(
                icon=new_icon,
                theme_text_color="Primary",
                size_hint=(None, None),
                size=(dp(24), dp(24)),
                pos_hint={'center_y': 0.5,}
            )
            self.add_widget(self.arrow_icon)

        Window.bind(on_resize=self._on_window_resize)

    def _get_truncated_text(self, text):
        window_width = Window.width
        max_chars = int(window_width / 20)  # Dynamic character limit based on window width
        return text if len(text) <= max_chars else text[:max_chars] + '..'

    def _on_window_resize(self, *args):
        self.title_item.text = self._get_truncated_text(self.original_text)


    def on_enter(self, *args):
        if not self.header and self != global_active_button:
            self.md_bg_color = self.hover_bg_color

    def on_leave(self, *args):
        if not self.header:
            if self == global_active_button:
                self.md_bg_color = self.active_bg_color
            else:
                self.md_bg_color = self.default_bg_color

    def on_press(self):
        global global_active_button
        if not self.active_header and not self.header:
            if global_active_button:
                global_active_button.md_bg_color = global_active_button.default_bg_color
            global_active_button = self
            self.md_bg_color = self.active_bg_color
        
        # Ensure view switching happens for both header and non-header items
        if not self.active_header:
            print(f"Switching to view: {self.view_name}")
            # Explicitly check if switch_view_callback is a method
            if hasattr(self.switch_view_callback, '__call__'):
                try:
                    self.switch_view_callback(self.view_name,self.icon_text)
                except Exception as e:
                    print(f"Error switching view: {e}")
            else:
                print(f"switch_view_callback is not callable: {type(self.switch_view_callback)}")





class CustomExpansionPanel(BoxLayout):
    def __init__(self, title, icon, children,view_name, switch_view_callback, **kwargs):
        super().__init__(**kwargs)
        self.orientation = 'vertical'
        self.size_hint_y = None
        self.height = dp(50)  # Initial height of the header
        self.is_expanded = False
        self.children_items = children  
        self.active_header = False
        self.reset_global_bg = (0,0,0,0)
        
        self.header = ClickableIconButton(
            icon_text=icon,
            title=title,
            header=True,
            view_name=view_name,
            active_header=len(self.children_items) > 0,
            switch_view_callback=switch_view_callback,
            reset_global_bg=self.reset_global_bg,


        )
        self.header.bind(on_press=self.toggle_expansion)
        self.add_widget(self.header)
        
        self.content_layout = BoxLayout(
            orientation='vertical',
            size_hint_y=None,
            height=0  # Initially collapsed
        )
        self.add_widget(self.content_layout)

    # def on_child_press(self, title, callback):
    #     print(title)
    #     if callback:
    #         callback(title)

    def collapse(self):
        if self.is_expanded:
            self.is_expanded = False
            anim = Animation(height=0, duration=0.3)
            anim.start(self.content_layout)
            self.header.icon.theme_text_color = "Primary"
            self.header.md_bg_color = (0, 0, 0, 0)
            if hasattr(self.header, 'arrow_icon'):
                self.header.arrow_icon.icon = "plus-circle"
            self.content_layout.clear_widgets()
            anim_panel = Animation(height=dp(50), duration=0.3)
            anim_panel.start(self)


    def toggle_expansion(self, *args):
        global current_expanded_panel
        
        if current_expanded_panel and current_expanded_panel != self:
            current_expanded_panel.collapse()
        
        self.is_expanded = not self.is_expanded
        
        if self.is_expanded:
            current_expanded_panel = self
            for child_title, child_icon, child_view in self.children_items:
                child_item = ClickableIconButton(
                    icon_text=child_icon,
                    title=child_title,
                    header=False,
                    active_header=False,
                    view_name=child_view,
                    # Directly pass the switch_view_callback from the parent
                    switch_view_callback=self.header.switch_view_callback,
                    reset_global_bg=self.reset_global_bg,
                )
                self.content_layout.add_widget(child_item)
        
        # Rest of the method remains the same...

            
            total_height = sum(child.height for child in self.content_layout.children)
            anim_content = Animation(height=total_height, duration=0.3)
            anim_content.start(self.content_layout)
            
            panel_height = self.header.height + total_height
            anim_panel = Animation(height=panel_height, duration=0.3)
            anim_panel.start(self)
            self.header.md_bg_color =(0.8, 0.8, 0.8, 1) 
            self.header.icon.theme_text_color = "Secondary"
            if hasattr(self.header, 'arrow_icon'):
                self.header.arrow_icon.icon = "minus-circle"
            
        else:
            current_expanded_panel = None
            anim_content = Animation(height=0, duration=0.3)
            anim_content.start(self.content_layout)
            
            anim_panel = Animation(height=dp(50), duration=0.3)
            anim_panel.start(self)
            self.header.md_bg_color = (0, 0, 0, 0)
            self.header.icon.theme_text_color = "Primary"
            if hasattr(self.header, 'arrow_icon'):
                self.header.arrow_icon.icon = "plus-circle"
            
            self.content_layout.clear_widgets()

   