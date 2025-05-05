from kivy.uix.boxlayout import BoxLayout
from kivy.metrics import dp

class ViewManager:
    def __init__(self, display_body):
        self.display_body = display_body
        self.current_view = None
    
    def switch_view(self, view_name, view_class):
        """Handle view switching"""
        try:
            # Clear current view
            self.display_body.clear_widgets()
            
            # Create and configure new view
            new_view = view_class()
            new_view.size_hint = (1, 1)
            
            # Add to display body
            self.display_body.add_widget(new_view)
            self.current_view = new_view
            
            return True
        except Exception as e:
            print(f"Error switching view: {e}")
            return False
