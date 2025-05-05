from kivy.uix.boxlayout import BoxLayout
from kivy.metrics import dp
from kivy.uix.behaviors import ButtonBehavior
from kivy.graphics import Color, Rectangle, RoundedRectangle, Line
from kivy.animation import Animation
from kivy.uix.label import Label
from kivy.uix.relativelayout import RelativeLayout
from kivy.uix.widget import Widget
from kivy.graphics import Point
from kivy_garden.graph import Graph, MeshLinePlot
from kivy.uix.scrollview import ScrollView
from kivy.graphics import Color, Line 
from kivy.core.window import Window
from data.lineChart_data import dummy_data


global activteTime
global activteYear

class CustomTimeButnton(ButtonBehavior, BoxLayout):
    def __init__(self, text, selected=False, **kwargs):
        super().__init__(**kwargs)
        self.orientation = 'horizontal'
        self.size_hint = (None, None)
        self.size = (dp(60), dp(20))
        self.selected = selected
        self.pos_hint = {'center_y': 0.5}
        self.opacity = 1

        self.label = Label(
            text=text,
            font_size='12sp',
            color=[1, 1, 1, 1],  # White text color
            size_hint=(1, None),
            height=dp(20),
            halign='center',
            valign='center'  # Add vertical centering
        )
        # Enable text centering
        self.label.bind(size=self.label.setter('text_size'))
        
        self.add_widget(self.label)
        
        self.bind(pos=self.update_canvas)
        self.bind(size=self.update_canvas)
        
        self.update_canvas()
        
    def update_canvas(self, *args):
            self.canvas.before.clear()
            with self.canvas.before:
                # Background
                Color(*(self.get_color()))
                # Set different radius based on text
                if self.label.text == 'weekly':
                    radius = [20, 0, 0, 20]  # Left side rounded
                elif self.label.text == 'yearly':
                    radius = [0, 20, 20, 0]  # Right side rounded
                else:
                    radius = [0, 0, 0, 0]    # No radius for monthly
                    
                RoundedRectangle(
                    pos=self.pos,
                    size=self.size,
                    radius=radius
                )
                
                # Border with same radius pattern
                Color(1, 1, 1, 0.5)  # White border with 50% opacity
                Line(
                    width=1,
                    rounded_rectangle=(
                        self.pos[0], self.pos[1],
                        self.size[0], self.size[1],
                        radius[0], radius[1], radius[2], radius[3]
                    )
                )
    
    def get_color(self):
        return [1, 1, 1, 0.08] if self.selected else [0, 0, 0, 0]
    
    def on_press(self):
            anim = Animation(opacity=0.7, duration=0.1)
            anim.start(self)     
    
    def on_release(self):
            anim = Animation(opacity=1, duration=0.1)
            anim.start(self)

    def set_selected(self, selected):
            self.selected = selected
            self.update_canvas()


class CustomYearButton(ButtonBehavior, BoxLayout):
    def __init__(self, text, selected=False, **kwargs):
        super().__init__(**kwargs)
        self.orientation = 'horizontal'
        self.size_hint = (None, None)
        self.size = (dp(60), dp(20))
        self.pos_hint = {'center_y': 0.5}
        self.selected = selected
        self.opacity = 1
        
        # Create the label with updated properties
        self.label = Label(
            text=text,
            font_size='12sp',
            color=[1, 1, 1, 1],  # White text color
            size_hint=(1, None),
            height=dp(20),
            halign='center',
            valign='center'  # Add vertical centering
        )
        # Enable text centering
        self.label.bind(size=self.label.setter('text_size'))
        
        self.add_widget(self.label)
        
        self.bind(pos=self.update_canvas)
        self.bind(size=self.update_canvas)
        
        self.update_canvas()
        
    def update_canvas(self, *args):
        try:
            self.canvas.before.clear()
            with self.canvas.before:
                Color(*(self.get_color()))
                RoundedRectangle(
                    pos=self.pos,
                    size=self.size,
                    radius=[5,]
                )
                Color(1, 1, 1, 0.5)  # White border with 10% opacity
                Line(
                    rounded_rectangle=(
                        self.pos[0], self.pos[1],
                        self.size[0], self.size[1],
                        5  # radius
                    ),
                    width=1  # border width
                )
        except Exception as e:
            print(f"Error updating canvas: {e}")
    
    def get_color(self):
        # Return mirror white with 8% opacity if selected, transparent if not
        return [1, 1, 1, 0.08] if self.selected else [0, 0, 0, 0]
    
    def on_press(self):
            anim = Animation(opacity=0.7, duration=0.1)
            anim.start(self)

        
    def on_release(self):
            anim = Animation(opacity=1, duration=0.1)
            anim.start(self)
            
            # Find LineChartWidget parent
            parent = None
            current = self
            while current and not isinstance(current, LineChartWidget):
                current = current.parent
            parent = current
            
            if not parent:
                print("Error: Could not find LineChartWidget parent")
                return
                
            # Update graph with selected year data
            if parent.period == 'Yearly':  # Check period instead of period_spinner
                parent.update_yearly_graph(self.label.text)
                
                # Update selection state
                for selector in parent.year_selectors:
                    selector.set_selected(selector == self)
        
    def set_selected(self, selected):
            self.selected = selected
            self.update_canvas()
            print(f"Error in set_selected: {e}")

class ColorLabelContainer(BoxLayout):
    def __init__(self, color, label_text, **kwargs):
        super().__init__(**kwargs)
        self.orientation = 'horizontal'
        self.spacing = dp(5)
        self.size_hint = (1, None)
        self.size = (0, dp(15))
        self.dot_color = color
        self.pos_hint = {'center_X': 0.5}

        # Create dot container
        self.dot_widget = Widget(
            size_hint=(None, None),
            size=(dp(20), dp(20))
        )
        
        with self.dot_widget.canvas:
            Color(*self.dot_color)
            Point(points=(self.dot_widget.center_x, self.dot_widget.center_y), pointsize=5)
            
        self.dot_widget.bind(pos=self.update_dot, size=self.update_dot)
        
        # Add label
        self.label = Label(
            text=label_text,
            font_size='10sp',
            color=[0.2, 0.2, 0.2, 0.8],
            size_hint=(None, None),
            height=dp(30)
        )

        self.add_widget(self.dot_widget)
        self.add_widget(self.label)

    def update_dot(self, *args):
        self.dot_widget.canvas.clear()
        with self.dot_widget.canvas:
            Color(*self.dot_color)
            Point(points=(self.dot_widget.center_x, self.dot_widget.center_y), pointsize=5)


class LineChartWidget(BoxLayout):
    def __init__(self, period='weekly', **kwargs):
        super().__init__(**kwargs)
        self.orientation = 'vertical'
        self.size_hint_y = None
        self.height = dp(350)
        self.padding = dp(2)
        self.period = period

        # Create nav section
        self.nav_section = BoxLayout(
            orientation='vertical',
            size_hint_y=None,
            height=dp(80),  # Increased to accommodate both selectors
            padding=[dp(5)]  # Add horizontal padding
        )
        self.nav_section.bind(pos=self.update_section_bg, size=self.update_section_bg)
        # Create graph section
        self.graph_section = BoxLayout(
            orientation='vertical',
            size_hint_y=None,
            height=dp(270),  # Adjusted to maintain total height
            padding=[dp(10), dp(5)]
        )
        self.graph_section.bind(pos=self.update_section_bg, size=self.update_section_bg)

################Top Section ################

        self.time_container = BoxLayout(
            orientation='horizontal',
            size_hint=(None, None),
            spacing=dp(3),
            height=dp(35),
            padding=[dp(5), dp(5)],  
            pos_hint={'center_x': 0.5}  
        )
        self.time_container.bind(pos=self.update_container_bg, size=self.update_container_bg)
        
        # Add time periods to container
        times = list(dummy_data.keys())
        self.time_selectors = []
        for period in times:
            time_selector = CustomYearButton(
                text=period,
                selected=(period == times[0]),
            )
            self.time_selectors.append(time_selector)
            self.time_container.add_widget(time_selector)
            
        # Calculate container width based on children
        self.time_container.size_hint_x = None
        self.time_container.width = len(times) * (dp(60) + dp(5)) 

        self.nav_section.add_widget(self.time_container)
        

     # Create scroll view for year selectors
        self.year_scroll = ScrollView(
            size_hint=(None, 1),  # Full height, width controlled
            size=(dp(300), 0),    # Initial height is 0
            do_scroll_y=False,
            do_scroll_x=True,
            bar_width=0,          # Hide scrollbar
            opacity=0,            # Initially hidden
            pos_hint={'center_x': 0.5}
        )
        # Create container for year selectors with background
        self.year_container = BoxLayout(
            orientation='horizontal',
            size_hint=(None, None),
            height=dp(35),
            spacing=dp(5),
            padding=[dp(5), 0]
        )
        
        # Bind update_container_bg to year_container
        self.year_container.bind(pos=self.update_container_bg, size=self.update_container_bg)
        
        # Add years to container
        years = list(dummy_data['Yearly'].keys())
        self.year_selectors = []
        for year in years:
            year_selector = CustomTimeButnton(
                text=year,
                selected=(year == years[0])
            )
            self.year_selectors.append(year_selector)
            self.year_container.add_widget(year_selector)
            
        # Calculate container width based on children
        self.year_container.size_hint_x = None
        self.year_container.width = len(years) * (dp(60) + dp(10))  # width + spacing + padding
        self.year_container.bind(minimum_width=self.year_container.setter('width'))
        
        # Add container to scroll view and bind to window size
        self.year_scroll.add_widget(self.year_container)
        self.nav_section.add_widget(self.year_scroll)
        Window.bind(on_resize=self.update_scroll_width)

        # Add sections to main widget
        self.add_widget(self.nav_section)
        self.add_widget(self.graph_section)



################Graph Section ################

    #     selected_data = this is fro data to show in the graph it must be update when custom time or year is changed

    #     self.float_box = BoxLayout(
    #         orientation='vertical',
    #         size_hint=(None, None),
    #         width=dp(120), # Adjust size as needed
    #         pos_hint={'right': 0.993, 'top': 0.98},
    #         padding=dp(5)
    #     )

    #     for data in color_datas:
    #         color_label = ColorLabelContainer(
    #             color=data['color'],
    #             label_text=data['label']
    #         )
    #         self.float_box.add_widget(color_label)
        
    #     self.float_box.height = (len(color_datas) * dp(20))

    #     # Create main_container for graph and controls
    #     self.main_container = BoxLayout(
    #         orientation='vertical',
    #         size_hint_y=None,
    #         height=dp(270)  # Same as graph_section height
    #     )

    #     #Create the graph widget
    #     self.graph = Graph(
    #         xlabel='X',
    #         ylabel='Y',
    #         x_ticks_minor=5,
    #         x_ticks_major=1,
    #         y_ticks_major=1,
    #         y_grid_label=True,
    #         x_grid_label=True,
    #         padding=5,
    #         x_grid=True,
    #         y_grid=True,
    #         xmin=0,
    #         xmax=100,
    #         ymin=0,
    #         ymax=100,
    #         size_hint=(1, None),
    #         height=dp(270),
    #         border_color=[1, 1, 1, 0.3],  # White with low opacity
    #         tick_color=[1, 1, 1, 0.3],    # White with low opacity
    #         label_options={
    #             'color': [1, 1, 1, 0.7],  # White with medium opacity
    #             'bold': True
    #         }
    #     )

    #     # Add graph to main_container
    #     self.main_container.add_widget(self.graph)

    #     # Create root_layout
    #     self.root_layout = RelativeLayout()
        
    #     # Add main_container to root_layout first
    #     self.root_layout.add_widget(self.main_container)  # Add this line
        
    #     # Then add float_box
    #     self.root_layout.add_widget(self.float_box)
        
    #     # Finally add root_layout to graph_section
    #     self.graph_section.add_widget(self.root_layout)

    #     self.float_box.bind(pos=self.update_rect, size=self.update_rect)




    # # Add method to handle year selection
    # def on_year_selected(self, selected_year):
    #     try:
    #         # Update selectors first
    #         for selector in self.year_selectors:
    #             if selector.label.text == selected_year:
    #                 selector.set_selected(True)
    #                 self.update_yearly_graph(selected_year)
    #             else:
    #                 selector.set_selected(False)
    #     except Exception as e:
    #         print(f"Error in year selection: {e}")

    # def on_period_change(self, spinner, text):
    #     try:
    #         # Show/hide year selector based on period
    #         if text == 'Yearly':
    #             anim = Animation(opacity=1, height=dp(35), duration=0.3)
    #             anim.start(self.year_scroll)
    #             # Reset year selector to 'All' when switching to yearly view
    #             self.on_year_selected('All')
    #         else:
    #             anim = Animation(opacity=0, height=0, duration=0.3)
    #             anim.start(self.year_scroll)

    #         # Update graph data based on period
    #         self.update_graph_data(text)
    #     except Exception as e:
    #         print(f"Error in period change: {e}")

    # def update_graph_data(self, period):
    #     # Update x-axis range and labels based on period
    #     if period == 'Weekly':
    #         self.graph.xmax = 6
    #         self.graph.x_ticks_major = 1
    #         self.graph.x_ticks_minor = 0
    #         self.graph.xlabel = 'Days of Week'
    #         self.graph.x_grid_label = True
    #         self.days = ['M', 'T', 'W', 'T', 'F', 'S', 'S']
                                                
    #     elif period == 'Monthly':
    #         self.graph.xmax = 11
    #         self.graph.x_ticks_major = 1
    #         self.graph.x_ticks_minor = 0
    #         self.graph.xlabel = 'Months'
    #         self.graph.x_grid_label = True
    #         self.months = ['J', 'F', 'M', 'A', 'M', 'J', 'J', 'A', 'S', 'O', 'N', 'D']
            
    #     else:  # Yearly
    #         self.graph.xmax = len(self.yearly_data) - 1
    #         self.graph.x_ticks_major = 1
    #         self.graph.x_ticks_minor = 0
    #         self.graph.xlabel = 'Years'
    #         self.graph.x_grid_label = True
    #         self.years = list(sorted(self.yearly_data.keys()))[1:]

    #     # Update plots with new data and line width
    #     try:
    #         for category, plot in self.plots.items():
    #             plot.points = self.dummy_data[period][category]
    #             plot.line_width = 1.5

    #         # Format axis labels
    #         self.graph.x_grid = True
    #         self.graph.y_grid = True
    #         self.graph.grid_color = [1, 1, 1, 0.1]
    #         self.graph.border_color = [1, 1, 1, 0.3]
    #         self.graph.tick_color = [1, 1, 1, 0.3]

    #         # Update y-axis range based on max sales
    #         max_sales = max(max(point[1] for point in self.dummy_data[period][category]) 
    #                       for category in self.dummy_data[period])
    #         self.graph.ymax = max_sales * 1.2
    #         self.graph.y_ticks_major = max_sales / 5

    #         # Format y-axis label
    #         formatted_max = f"${max_sales/1000:.0f}k" if max_sales >= 1000 else f"${max_sales:.0f}"
    #         self.graph.ylabel = f'Sales (0-{formatted_max})'
    #     except Exception as e:
    #         print(f"Error updating graph data: {e}")

    # def update_yearly_graph(self, selected_year):
    #     try:
    #         # Return early if not in yearly mode
    #         if self.period != 'Yearly':
    #             return

    #         # Get sorted years excluding 'All'
    #         years = sorted(year for year in self.yearly_data.keys() if year != 'All')
            
    #         # Update plots for each category
    #         for category in self.plots:
    #             points = []
    #             if selected_year == 'All':
    #                 # Show all years
    #                 for year in years:
    #                     year_index = years.index(year)
    #                     sales = self.yearly_data[year][category]['sales']
    #                     points.append((year_index, sales))
    #             else:
    #                 # Show only selected year
    #                 year_index = years.index(selected_year)
    #                 sales = self.yearly_data[selected_year][category]['sales']
    #                 points = [(year_index, sales)]
                
    #             self.plots[category].points = points

    #         # Update graph x-axis settings
    #         self.graph.xmin = 0
    #         self.graph.xmax = len(years) - 1
    #         self.graph.x_ticks_major = 1
    #         self.graph.x_ticks_minor = 0

    #         # Calculate max sales for y-axis scaling
    #         if selected_year == 'All':
    #             max_sales = max(
    #                 data[category]['sales'] 
    #                 for data in self.yearly_data.values() 
    #                 for category in data
    #             )
    #         else:
    #             max_sales = max(
    #                 self.yearly_data[selected_year][category]['sales']
    #                 for category in self.yearly_data[selected_year]
    #             )

    #         # Update graph y-axis settings
    #         self.graph.ymax = max_sales * 1.2  # Add 20% padding
    #         self.graph.y_ticks_major = max_sales / 5
    #         formatted_max = f"${max_sales/1000:.0f}k" if max_sales >= 1000 else f"${max_sales:.0f}"
    #         self.graph.ylabel = f'Sales (0-{formatted_max})'

    #     except Exception as e:
    #         print(f"Error updating yearly graph: {e}")
    #         # Optionally log the full traceback for debugging
    #         import traceback
    #         traceback.print_exc()




    # # Add the update_container_bg method to draw backgrounds
    def update_container_bg(self, instance, value):
        instance.canvas.before.clear()
        with instance.canvas.before:
            Color(1, 1, 1, 0.05)  # Very subtle white background
            RoundedRectangle(
                pos=instance.pos,
                size=instance.size,
                radius=[dp(10)]  # Rounded corners
            )
            
    def update_scroll_width(self, instance, width, height):
        """Update scroll view width based on window size"""
        max_width = min(dp(300), width * 0.8)  # 80% of window width or 300dp
        self.year_scroll.width = max_width

    def update_section_bg(self, instance, value):
    
        try:
            instance.canvas.before.clear()
            with instance.canvas.before:
                Color(1, 1, 1, 0.03)  # Very subtle white background
                RoundedRectangle(
                    pos=instance.pos,
                    size=instance.size,
                    radius=[dp(10)]  # Rounded corners
                )
                # Add border
                Color(1, 1, 1, 0.1)  # Subtle white border
                Line(
                    rounded_rectangle=(
                        instance.pos[0], instance.pos[1],
                        instance.size[0], instance.size[1],
                        dp(10)
                    ),
                    width=1
                )
        except Exception as e:
            print(f"Error updating section background: {e}")

    def update_rect(self, *args):
        self.main_container.canvas.before.clear()
        with self.main_container.canvas.before:
            Color(1, 1, 1, 0.5)
            Rectangle(pos=self.main_container.pos, size=self.main_container.size)
                
        self.float_box.canvas.before.clear()
        with self.float_box.canvas.before:
            Color(1, 1, 1, 0.3) 
            RoundedRectangle(
                pos=self.float_box.pos, 
                size=self.float_box.size,
                radius=[dp(5)]
            )
