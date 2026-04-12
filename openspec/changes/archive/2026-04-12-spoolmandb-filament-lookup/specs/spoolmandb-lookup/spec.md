## ADDED Requirements

### Requirement: SpoolmanDB data is fetched and cached locally
The system SHALL fetch the SpoolmanDB filament database from `https://donkie.github.io/SpoolmanDB/filaments.json` and cache it in browser localStorage under the key `spoolmandb_cache`. The cache entry SHALL store the parsed data array, the HTTP ETag from the response, and the fetch timestamp. The system SHALL serve cached data without a network request if the cache is less than 24 hours old. The system SHALL re-fetch using `If-None-Match` when the cache is stale; a 304 response SHALL refresh the timestamp without replacing data. If a network fetch fails and a cache entry exists, the system SHALL use the stale cached data regardless of age.

#### Scenario: First fetch populates cache
- **WHEN** no `spoolmandb_cache` entry exists in localStorage
- **THEN** the system fetches filaments.json, parses the JSON array, and stores `{ data, etag, fetched_at }` in localStorage

#### Scenario: Fresh cache skips network
- **WHEN** `spoolmandb_cache` exists and `fetched_at` is within 24 hours
- **THEN** the system uses the cached data without making a network request

#### Scenario: Stale cache triggers conditional re-fetch
- **WHEN** `spoolmandb_cache` exists but `fetched_at` is older than 24 hours
- **THEN** the system sends a GET request with `If-None-Match: <cached etag>`

#### Scenario: 304 response refreshes timestamp
- **WHEN** the server returns HTTP 304 Not Modified
- **THEN** the cached data is retained and `fetched_at` is updated to now

#### Scenario: 200 response replaces cache
- **WHEN** the server returns HTTP 200 with updated data
- **THEN** the cache is replaced with the new data, new etag, and current timestamp

#### Scenario: Network failure with existing cache
- **WHEN** the fetch request fails (network error or non-200/304 status) and a cache entry exists
- **THEN** the system uses the stale cached data and does not surface an error to the user

#### Scenario: Network failure without cache
- **WHEN** the fetch request fails and no cache entry exists
- **THEN** the search panel displays a "database unavailable" message

---

### Requirement: SpoolmanDB search panel appears on filament and spool create/edit forms
The system SHALL display an inline "Search filament database" panel above the form fields on the Filament Create, Filament Edit, and Spool Create pages. The panel SHALL contain a text input. The panel MAY be collapsed/hidden by default and expanded by the user.

#### Scenario: Panel is present on Filament Create
- **WHEN** the user navigates to the New Filament page
- **THEN** a SpoolmanDB search panel is visible above the form fields

#### Scenario: Panel is present on Filament Edit
- **WHEN** the user navigates to the Edit Filament page
- **THEN** a SpoolmanDB search panel is visible above the form fields

#### Scenario: Panel is present on Spool Create
- **WHEN** the user navigates to the New Spool page
- **THEN** a SpoolmanDB search panel is visible above the form fields

---

### Requirement: Search filters SpoolmanDB entries client-side
As the user types in the search input, the system SHALL filter the locally cached SpoolmanDB entries and display up to 10 matching results. Filtering SHALL be case-insensitive and SHALL match against manufacturer name, material string, and color/variant name. Each result SHALL display as "Manufacturer · Material · Color name".

#### Scenario: Typing filters results
- **WHEN** the user types a query into the search input
- **THEN** the results list updates immediately to show up to 10 matching SpoolmanDB entries

#### Scenario: Empty query shows no results
- **WHEN** the search input is empty
- **THEN** no results are displayed

#### Scenario: No matches shows empty state
- **WHEN** the query matches no SpoolmanDB entries
- **THEN** a "No results" message is displayed

---

### Requirement: Selecting a SpoolmanDB entry auto-fills filament form fields
When a user selects a SpoolmanDB entry from search results on the Filament Create or Filament Edit form, the system SHALL populate the following fields: manufacturer, material (base type), material modifier, diameter, density, print temp, and bed temp. Fields SHALL remain editable after auto-fill. The material string SHALL be parsed by stripping known composite suffixes (`+`, `-CF`, `-GF`, `-HF`, `-ESD`) to derive a base `MaterialType` and an optional `material_modifier`.

#### Scenario: Auto-fill simple material
- **WHEN** the user selects an entry with `material: "PLA"` on the Filament Create form
- **THEN** the Material field is set to `PLA` and the Modifier field remains empty

#### Scenario: Auto-fill composite material
- **WHEN** the user selects an entry with `material: "PETG-CF"` on the Filament Create form
- **THEN** the Material field is set to `PETG` and the Modifier field is set to `CF`

#### Scenario: Auto-fill physical properties
- **WHEN** the user selects an entry with defined density, diameter, extruder_temp, and bed_temp
- **THEN** the corresponding form fields are populated with those values

#### Scenario: Fields remain editable after auto-fill
- **WHEN** form fields have been auto-filled from a SpoolmanDB selection
- **THEN** the user can still modify any field before submitting

---

### Requirement: Selecting a SpoolmanDB entry auto-fills spool color and weight fields
When a user selects a SpoolmanDB entry from search results on the Spool Create form, the system SHALL populate the color picker with the entry's `color_hex`, the color name field with the entry's `name`, and the net weight field with the entry's `weight` value (in grams).

#### Scenario: Auto-fill color
- **WHEN** the user selects an entry with a defined `color_hex` on the Spool Create form
- **THEN** the color picker is set to that hex value

#### Scenario: Auto-fill color name
- **WHEN** the user selects an entry with a defined `name`
- **THEN** the Color name field is set to that value

#### Scenario: Auto-fill net weight
- **WHEN** the user selects an entry with a defined `weight`
- **THEN** the Net weight field is set to that value

---

### Requirement: Spool Create auto-creates a missing filament when an entry is selected
When the user selects a SpoolmanDB entry on the Spool Create form and no existing filament matches the entry's manufacturer, material, and diameter (within ±0.01mm, case-insensitive manufacturer), the system SHALL automatically create a new filament via the API using the entry's physical properties and SHALL display a dismissable notification informing the user that a filament was created. The new filament SHALL be selected in the filament dropdown.

#### Scenario: Matching filament found — no auto-create
- **WHEN** the user selects a SpoolmanDB entry and a filament with matching manufacturer, material, and diameter already exists
- **THEN** no new filament is created and the existing filament is selected in the dropdown

#### Scenario: No matching filament — auto-create triggers
- **WHEN** the user selects a SpoolmanDB entry and no matching filament exists
- **THEN** a new filament is created via POST /api/v1/filament with the entry's properties

#### Scenario: Auto-create notification shown
- **WHEN** a filament is auto-created
- **THEN** a dismissable info banner is shown: "Filament '[Manufacturer] [Material]' was created automatically."

#### Scenario: Auto-created filament is selected in dropdown
- **WHEN** a filament is auto-created
- **THEN** the filament dropdown is set to the newly created filament's id
