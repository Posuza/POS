dummy_data = {
    'Weekly': {
        'All': [(x, x*10) for x in range(7)],
        'Food': [(x, x*8) for x in range(7)],
        'Drink': [(x, x*6) for x in range(7)],
        'grocery': [(x, x*4) for x in range(7)]

    },
    'Monthly': {
        'All': [(x, x*50) for x in range(30)],
        'Food': [(x, x*40) for x in range(30)],
        'Drink': [(x, x*30) for x in range(30)],
        'grocery': [(x, x*20) for x in range(30)]
    },
    'Yearly':{
        'All':{
            'All': [(x, x*200) for x in range(4)],  # 4 years of data
            'Food': [(x, x*150) for x in range(4)],
            'Drink': [(x, x*100) for x in range(4)],
            'grocery': [(x, x*75) for x in range(4)]
        },
        '2022': {
            'All': [(x, x*50) for x in range(12)],
            'Food': [(x, x*40) for x in range(12)],
            'Drink': [(x, x*30) for x in range(12)],
            'grocery': [(x, x*20) for x in range(12)]
        },
        '2023': {
            'All': [(x, x*50) for x in range(12)],
            'Food': [(x, x*40) for x in range(12)],
            'Drink': [(x, x*30) for x in range(12)],
            'grocery': [(x, x*20) for x in range(12)]
        },
        '2024': {
            'All': [(x, x*50) for x in range(12)],
            'Food': [(x, x*40) for x in range(12)],
            'Drink': [(x, x*30) for x in range(12)],
            'grocery': [(x, x*20) for x in range(12)]
        },
        '2025': {
            'All': [(x, x*50) for x in range(12)],
            'Food': [(x, x*40) for x in range(12)],
            'Drink': [(x, x*30) for x in range(12)],
            'grocery': [(x, x*20) for x in range(12)]
        } }
    }