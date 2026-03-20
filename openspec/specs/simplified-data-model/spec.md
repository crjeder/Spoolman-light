# Simplified Data Model Spec

## Requirements

### Requirement: Filament is a material formula only
The system SHALL model Filament as a set of printing/material properties only. Filament SHALL NOT carry color, price, weight, spool weight, article number, or external ID fields.

#### Scenario: Filament fields are limited to material properties
- **WHEN** a Filament is created or retrieved
- **THEN** its fields are: id, registered, vendor (str), name, material, density, diameter, extruder_temp, bed_temp, comment, extra

#### Scenario: Creating a filament does not require color or price
- **WHEN** a POST request to `/filament` is made with only density and diameter
- **THEN** the filament is created successfully with no color, price, or weight fields

### Requirement: Vendor is a free-text string on Filament
The system SHALL store vendor as an optional free-text string directly on the Filament entity. There SHALL be no separate Vendor entity, no vendor CRUD endpoints, and no vendor foreign key.

#### Scenario: Filament can be created with a vendor string
- **WHEN** a POST request includes `"vendor": "eSun"`
- **THEN** the filament is stored with vendor "eSun" and retrievable via GET

#### Scenario: Filament can be filtered by vendor string
- **WHEN** a GET request to `/filament` includes `?vendor=eSun`
- **THEN** only filaments with vendor matching "eSun" are returned

#### Scenario: No vendor endpoints exist
- **WHEN** any request is made to `/vendor/*`
- **THEN** the server returns 404

### Requirement: Spool carries all physical instance attributes
The system SHALL model Spool as the physical object, carrying color, weight, spool weight, price, and usage data. Spool SHALL store color fields that were previously on Filament.

#### Scenario: Spool can be created with color
- **WHEN** a POST request to `/spool` includes `color_hex`, `initial_weight`, and `price`
- **THEN** the spool is created and those fields are stored and returned

#### Scenario: Two spools of the same filament can have different colors
- **WHEN** two spools are created with the same `filament_id` but different `color_hex` values
- **THEN** both spools are stored independently with their respective colors

#### Scenario: Two spools of the same filament can have different prices
- **WHEN** two spools are created with the same `filament_id` but different `price` values
- **THEN** both spools are stored independently with their respective prices

### Requirement: Spool color search
The system SHALL support finding spools by color similarity. Color search SHALL operate on Spool (not Filament) since color is now a spool attribute.

#### Scenario: Find spools by color similarity
- **WHEN** a GET request is made to `/spool/find-by-color` with a hex color query
- **THEN** spools whose color_hex or multi_color_hexes are within the similarity threshold are returned

### Requirement: Removed fields are rejected
The system SHALL return a validation error if a client submits fields that have been removed from the model.

#### Scenario: Filament creation rejects removed fields
- **WHEN** a POST to `/filament` includes `weight`, `spool_weight`, `price`, `color_hex`, `article_number`, or `external_id`
- **THEN** the server returns 422 Unprocessable Entity

#### Scenario: Spool creation rejects removed fields
- **WHEN** a POST to `/spool` includes `lot_nr` or `external_id`
- **THEN** the server returns 422 Unprocessable Entity
