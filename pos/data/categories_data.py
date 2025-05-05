from data.product_data import products

def get_unique_categories():
    # Get unique categories from products
    unique_cats = set(category for _,_, _, category, _ in products)
    return sorted(list(unique_cats))

def calculate_category_totals():
    quantities = {}
    # Count unique products in each category
    for _,name, _, category, _ in products:
        if category not in quantities:
            quantities[category] = set()
        quantities[category].add(name)
    
    # Convert sets to counts
    return {cat: len(products) for cat, products in quantities.items()}

# Calculate quantities (number of unique products)
category_quantities = calculate_category_totals()
total_quantity = sum(category_quantities.values())

# Create categories list with product counts
categories = [('All', total_quantity)]
for category in get_unique_categories():
    categories.append((category, category_quantities.get(category, 0)))
