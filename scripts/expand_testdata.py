import json

with open('assets/spoolman.json') as f:
    data = json.load(f)

filaments = data['filaments']
spools = data['spools']
locations = data['locations']

# Add 20 more filaments (IDs 1021-1040)
new_filaments = [
    {"id": 1021, "manufacturer": "Bambu", "material": "PETG", "material_modifier": "HF", "diameter": 1.75, "density": 1.27, "print_temp": 230, "bed_temp": 70, "min_print_temp": 220, "max_print_temp": 245, "min_bed_temp": 65, "max_bed_temp": 80, "registered": "2025-04-01T10:00:00Z", "comment": "High flow"},
    {"id": 1022, "manufacturer": "Prusament", "material": "PLA", "material_modifier": "Galaxy", "diameter": 1.75, "density": 1.24, "print_temp": 215, "bed_temp": 60, "min_print_temp": 205, "max_print_temp": 230, "min_bed_temp": 50, "max_bed_temp": 70, "registered": "2025-04-05T09:00:00Z", "comment": "Galaxy effect"},
    {"id": 1023, "manufacturer": "eSUN", "material": "ABS", "material_modifier": None, "diameter": 1.75, "density": 1.04, "print_temp": 240, "bed_temp": 100, "min_print_temp": 225, "max_print_temp": 260, "min_bed_temp": 90, "max_bed_temp": 110, "registered": "2025-03-15T11:00:00Z", "comment": None},
    {"id": 1024, "manufacturer": "Polymaker", "material": "ASA", "material_modifier": None, "diameter": 1.75, "density": 1.07, "print_temp": 250, "bed_temp": 100, "min_print_temp": 240, "max_print_temp": 265, "min_bed_temp": 90, "max_bed_temp": 110, "registered": "2025-03-20T12:00:00Z", "comment": None},
    {"id": 1025, "manufacturer": "ColorFabb", "material": "PETG", "material_modifier": "HT", "diameter": 1.75, "density": 1.27, "print_temp": 250, "bed_temp": 85, "min_print_temp": 240, "max_print_temp": 265, "min_bed_temp": 75, "max_bed_temp": 95, "registered": "2025-04-10T10:00:00Z", "comment": "High temperature"},
    {"id": 1026, "manufacturer": "Fiberlogy", "material": "PLA", "material_modifier": "HD", "diameter": 1.75, "density": 1.25, "print_temp": 210, "bed_temp": 60, "min_print_temp": 195, "max_print_temp": 225, "min_bed_temp": 50, "max_bed_temp": 70, "registered": "2025-04-12T09:00:00Z", "comment": "High detail"},
    {"id": 1027, "manufacturer": "Bambu", "material": "TPU", "material_modifier": "95A", "diameter": 1.75, "density": 1.22, "print_temp": 220, "bed_temp": 35, "min_print_temp": 210, "max_print_temp": 235, "min_bed_temp": None, "max_bed_temp": None, "registered": "2025-04-15T10:00:00Z", "comment": None},
    {"id": 1028, "manufacturer": "Sunlu", "material": "ABS", "material_modifier": None, "diameter": 1.75, "density": 1.04, "print_temp": 235, "bed_temp": 100, "min_print_temp": 220, "max_print_temp": 255, "min_bed_temp": 90, "max_bed_temp": 110, "registered": "2025-03-25T13:00:00Z", "comment": None},
    {"id": 1029, "manufacturer": "3DJake", "material": "PLA", "material_modifier": "Silk", "diameter": 1.75, "density": 1.24, "print_temp": 215, "bed_temp": 60, "min_print_temp": 200, "max_print_temp": 230, "min_bed_temp": 50, "max_bed_temp": 70, "registered": "2025-04-02T10:00:00Z", "comment": "Dual silk"},
    {"id": 1030, "manufacturer": "Overture", "material": "ABS", "material_modifier": None, "diameter": 1.75, "density": 1.04, "print_temp": 230, "bed_temp": 100, "min_print_temp": 220, "max_print_temp": 250, "min_bed_temp": 90, "max_bed_temp": 110, "registered": "2025-03-18T11:00:00Z", "comment": None},
    {"id": 1031, "manufacturer": "Prusament", "material": "PC", "material_modifier": None, "diameter": 1.75, "density": 1.20, "print_temp": 275, "bed_temp": 110, "min_print_temp": 260, "max_print_temp": 290, "min_bed_temp": 100, "max_bed_temp": 120, "registered": "2025-04-20T10:00:00Z", "comment": "Polycarbonate"},
    {"id": 1032, "manufacturer": "Fillamentum", "material": "PETG", "material_modifier": None, "diameter": 1.75, "density": 1.27, "print_temp": 240, "bed_temp": 80, "min_print_temp": 225, "max_print_temp": 255, "min_bed_temp": 70, "max_bed_temp": 90, "registered": "2025-04-18T09:00:00Z", "comment": None},
    {"id": 1033, "manufacturer": "ColorFabb", "material": "nGen", "material_modifier": None, "diameter": 1.75, "density": 1.19, "print_temp": 235, "bed_temp": 75, "min_print_temp": 220, "max_print_temp": 250, "min_bed_temp": 65, "max_bed_temp": 85, "registered": "2025-04-22T10:00:00Z", "comment": "Tough and flexible"},
    {"id": 1034, "manufacturer": "Hatchbox", "material": "PETG", "material_modifier": None, "diameter": 1.75, "density": 1.27, "print_temp": 235, "bed_temp": 80, "min_print_temp": 220, "max_print_temp": 250, "min_bed_temp": 70, "max_bed_temp": 90, "registered": "2025-04-08T11:00:00Z", "comment": None},
    {"id": 1035, "manufacturer": "Polymaker", "material": "PC", "material_modifier": "Max", "diameter": 1.75, "density": 1.19, "print_temp": 280, "bed_temp": 120, "min_print_temp": 265, "max_print_temp": 300, "min_bed_temp": 110, "max_bed_temp": 130, "registered": "2025-05-01T10:00:00Z", "comment": "High performance PC"},
    {"id": 1036, "manufacturer": "Bambu", "material": "PLA", "material_modifier": "Matte", "diameter": 1.75, "density": 1.24, "print_temp": 215, "bed_temp": 35, "min_print_temp": 190, "max_print_temp": 230, "min_bed_temp": 35, "max_bed_temp": 45, "registered": "2025-05-05T09:00:00Z", "comment": None},
    {"id": 1037, "manufacturer": "eSUN", "material": "TPU", "material_modifier": "87A", "diameter": 1.75, "density": 1.20, "print_temp": 215, "bed_temp": 30, "min_print_temp": 200, "max_print_temp": 230, "min_bed_temp": None, "max_bed_temp": None, "registered": "2025-05-08T10:00:00Z", "comment": "Extra flexible"},
    {"id": 1038, "manufacturer": "Prusament", "material": "FLEX", "material_modifier": None, "diameter": 1.75, "density": 1.22, "print_temp": 225, "bed_temp": 40, "min_print_temp": 215, "max_print_temp": 240, "min_bed_temp": None, "max_bed_temp": None, "registered": "2025-05-10T10:00:00Z", "comment": None},
    {"id": 1039, "manufacturer": "Fiberlogy", "material": "PETG", "material_modifier": "CF", "diameter": 1.75, "density": 1.35, "print_temp": 255, "bed_temp": 80, "min_print_temp": 245, "max_print_temp": 265, "min_bed_temp": 70, "max_bed_temp": 90, "registered": "2025-05-12T11:00:00Z", "comment": "Carbon fiber PETG"},
    {"id": 1040, "manufacturer": "3DJake", "material": "PLA", "material_modifier": "CF", "diameter": 1.75, "density": 1.30, "print_temp": 210, "bed_temp": 60, "min_print_temp": 200, "max_print_temp": 225, "min_bed_temp": 50, "max_bed_temp": 70, "registered": "2025-05-15T10:00:00Z", "comment": "Carbon fiber PLA"},
]
filaments.extend(new_filaments)

# Add 7 more locations (IDs 3009-3015)
new_locations = [
    {"id": 3009, "name": "Flexible Materials Box"},
    {"id": 3010, "name": "High-Temp Rack"},
    {"id": 3011, "name": "Carbon Fiber Shelf"},
    {"id": 3012, "name": "Printer 2 AMS"},
    {"id": 3013, "name": "Printer 3 AMS"},
    {"id": 3014, "name": "Archive Box"},
    {"id": 3015, "name": "Working Drawer"},
]
locations.extend(new_locations)

all_filament_ids = [f['id'] for f in filaments]
all_location_ids = [l['id'] for l in locations]

# Color palette with names
color_pool = [
    ({"r": 255, "g": 80, "b": 0, "a": 255}, "Orange"),
    ({"r": 0, "g": 0, "b": 0, "a": 255}, "Black"),
    ({"r": 255, "g": 255, "b": 255, "a": 255}, "White"),
    ({"r": 220, "g": 220, "b": 220, "a": 200}, "Clear"),
    ({"r": 30, "g": 30, "b": 200, "a": 255}, "Blue"),
    ({"r": 200, "g": 30, "b": 30, "a": 255}, "Red"),
    ({"r": 30, "g": 160, "b": 30, "a": 255}, "Green"),
    ({"r": 255, "g": 215, "b": 0, "a": 255}, "Gold"),
    ({"r": 180, "g": 0, "b": 200, "a": 255}, "Purple"),
    ({"r": 255, "g": 180, "b": 180, "a": 255}, "Pink"),
    ({"r": 50, "g": 200, "b": 200, "a": 255}, "Teal"),
    ({"r": 230, "g": 115, "b": 0, "a": 255}, "Copper"),
    ({"r": 192, "g": 192, "b": 192, "a": 255}, "Silver"),
    ({"r": 100, "g": 60, "b": 20, "a": 255}, "Brown"),
    ({"r": 0, "g": 200, "b": 100, "a": 255}, "Lime"),
    ({"r": 255, "g": 165, "b": 0, "a": 255}, "Amber"),
    ({"r": 70, "g": 130, "b": 180, "a": 255}, "Steel Blue"),
    ({"r": 50, "g": 50, "b": 50, "a": 255}, "Dark Gray"),
    ({"r": 255, "g": 255, "b": 0, "a": 255}, "Yellow"),
    ({"r": 135, "g": 206, "b": 235, "a": 255}, "Sky Blue"),
    ({"r": 255, "g": 20, "b": 147, "a": 255}, "Deep Pink"),
    ({"r": 0, "g": 128, "b": 128, "a": 255}, "Dark Teal"),
    ({"r": 139, "g": 0, "b": 0, "a": 255}, "Dark Red"),
    ({"r": 75, "g": 0, "b": 130, "a": 255}, "Indigo"),
    ({"r": 210, "g": 180, "b": 140, "a": 255}, "Tan"),
    ({"r": 64, "g": 224, "b": 208, "a": 255}, "Turquoise"),
    ({"r": 255, "g": 99, "b": 71, "a": 255}, "Tomato"),
    ({"r": 34, "g": 139, "b": 34, "a": 255}, "Forest Green"),
    ({"r": 128, "g": 0, "b": 128, "a": 255}, "Magenta"),
    ({"r": 0, "g": 0, "b": 128, "a": 255}, "Navy"),
]

dates_pool = [
    "2024-10-01T08:00:00Z", "2024-10-15T09:30:00Z", "2024-11-01T12:00:00Z", "2024-11-20T10:00:00Z",
    "2024-12-01T08:00:00Z", "2024-12-15T14:00:00Z", "2025-01-05T08:00:00Z", "2025-01-10T10:05:00Z",
    "2025-01-20T09:00:00Z", "2025-02-01T11:00:00Z", "2025-02-15T08:35:00Z", "2025-02-28T09:00:00Z",
    "2025-03-01T14:10:00Z", "2025-03-10T13:00:00Z", "2025-03-20T10:00:00Z", "2025-03-25T11:00:00Z",
    "2025-04-01T09:00:00Z", "2025-04-10T10:00:00Z", "2025-04-20T11:00:00Z", "2025-05-01T09:00:00Z",
    "2025-05-10T10:00:00Z", "2025-05-20T11:00:00Z", "2025-06-01T09:00:00Z", "2025-06-15T10:00:00Z",
    "2025-07-01T09:00:00Z", "2025-07-15T10:00:00Z", "2025-08-01T09:00:00Z", "2025-08-15T10:00:00Z",
    "2025-09-01T09:00:00Z", "2025-09-15T10:00:00Z",
]

# Generate 80 more spools (IDs 2121-2200)
spool_id = 2121
for i in range(80):
    color_entry, color_name = color_pool[i % len(color_pool)]
    filament_id = all_filament_ids[i % len(all_filament_ids)]

    # ~1-in-7 chance of no location
    location_id = None if i % 7 == 0 else all_location_ids[i % len(all_location_ids)]

    net_weight = 750.0 if i % 5 == 0 else 1000.0
    initial_weight = round(net_weight + 200.0 + (i % 5) * 5.0, 1)

    if i % 9 == 0:
        # Unopened
        current_weight = initial_weight
        first_used = None
        last_used = None
        comment = "Unopened"
        archived = False
    elif i % 11 == 0:
        # Nearly empty, archived
        current_weight = round(net_weight * 0.04, 1)
        first_used = dates_pool[(i + 3) % len(dates_pool)]
        last_used = dates_pool[(i + 8) % len(dates_pool)]
        comment = "Nearly empty"
        archived = True
    else:
        ratio = 0.2 + (i % 8) * 0.1
        current_weight = round(net_weight * ratio + 200.0, 1)
        first_used = dates_pool[(i + 2) % len(dates_pool)]
        last_used = dates_pool[(i + 5) % len(dates_pool)]
        misc_comments = [
            None, None, None, "Great quality", "Good adhesion",
            "Slightly damp - redry before use", "Perfect for functional parts",
            "Vibrant color", "Reserved for client project", "Test roll",
            "Backup spool", "Gift from friend",
        ]
        comment = misc_comments[i % len(misc_comments)]
        archived = False

    registered = dates_pool[i % len(dates_pool)]

    # ~1-in-13 chance of dual-color
    if i % 13 == 0 and i > 0:
        color2_entry, color2_name = color_pool[(i + 5) % len(color_pool)]
        colors = [color_entry, color2_entry]
        color_name = color_name + "/" + color2_name
    else:
        colors = [color_entry]

    spools.append({
        "id": spool_id,
        "filament_id": filament_id,
        "location_id": location_id,
        "colors": colors,
        "color_name": color_name,
        "initial_weight": initial_weight,
        "current_weight": current_weight,
        "registered": registered,
        "first_used": first_used,
        "last_used": last_used,
        "comment": comment,
        "archived": archived,
        "net_weight": net_weight,
    })
    spool_id += 1

print(f"Total filaments: {len(filaments)}")
print(f"Total spools: {len(spools)}")
print(f"Total locations: {len(locations)}")

data['filaments'] = filaments
data['spools'] = spools
data['locations'] = locations

with open('assets/spoolman.json', 'w') as f:
    json.dump(data, f, indent=2)

print("Written OK")
