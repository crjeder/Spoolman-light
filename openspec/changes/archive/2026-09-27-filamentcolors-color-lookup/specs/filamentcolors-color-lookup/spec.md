## ADDED Requirements

### Requirement: filamentcolors.xyz data is fetched and cached locally
The system SHALL fetch swatch data from the filamentcolors.xyz public API (`https://filamentcolors.xyz/api/swatch/`) directly via WASM fetch (no proxy) and cache the results in browser localStorage under the key `filamentcolors_cache`. The cache entry SHALL store the parsed swatch list, the source API's `db_version` (if provided) or fetch timestamp, and the fetch timestamp. The system SHALL serve cached data without a network request if the cache is less than 24 hours old. If a network fetch fails and a cache entry exists, the system SHALL use the stale cached data regardless of age. The system SHALL NOT issue more than one full-list fetch per 24-hour cache window, respecting the anonymous API's documented rate limit (100/min, 3600/hour per IP).

#### Scenario: First fetch populates cache
- **WHEN** no `filamentcolors_cache` entry exists in localStorage
- **THEN** the system fetches swatch data and stores `{ data, db_version, fetched_at }` in localStorage

#### Scenario: Fresh cache skips network
- **WHEN** `filamentcolors_cache` exists and `fetched_at` is within 24 hours
- **THEN** the system uses the cached data without making a network request

#### Scenario: Stale cache triggers re-fetch
- **WHEN** `filamentcolors_cache` exists but `fetched_at` is older than 24 hours
- **THEN** the system fetches fresh swatch data and replaces the cache

#### Scenario: Network failure with existing cache
- **WHEN** the fetch request fails and a cache entry exists
- **THEN** the system uses the stale cached data and does not surface an error to the user

#### Scenario: Network failure without cache
- **WHEN** the fetch request fails and no cache entry exists
- **THEN** the search panel displays an "unavailable" message

### Requirement: filamentcolors.xyz search panel on spool create/edit
The system SHALL display an inline "Search filamentcolors.xyz" panel on the Spool Create and Spool Edit forms, alongside the existing SpoolmanDB search panel. The panel SHALL contain a text input and, as the user types, SHALL filter the cached swatch list client-side (case-insensitive match against manufacturer and color name) and display up to 10 matching results, each showing a thumbnail, manufacturer, and color name. The panel is optional: the form SHALL remain fully usable without ever opening or using it.

#### Scenario: Panel is present on Spool Create
- **WHEN** the user navigates to the New Spool page
- **THEN** a "Search filamentcolors.xyz" panel is visible

#### Scenario: Panel is present on Spool Edit
- **WHEN** the user navigates to the Edit Spool page
- **THEN** a "Search filamentcolors.xyz" panel is visible

#### Scenario: Typing filters results
- **WHEN** the user types a query into the panel's search input
- **THEN** up to 10 matching swatches are listed, each with a thumbnail, manufacturer, and color name

#### Scenario: No matches shows empty state
- **WHEN** the query matches no cached swatches
- **THEN** a "No results" message is displayed

#### Scenario: Form usable without the panel
- **WHEN** the user fills in and submits the spool form without ever interacting with the filamentcolors.xyz panel
- **THEN** the spool is created or updated exactly as before this feature existed

### Requirement: Selecting a swatch applies its measured color and downloads its image
When the user selects a filamentcolors.xyz result, the system SHALL set the spool's color to the swatch's measured hex value (`hex_color` field) and SHALL request the server to download and store the swatch's image, associating the resulting image reference with the spool being created or edited. The color and color name fields SHALL remain editable after selection.

#### Scenario: Selecting a swatch sets the measured color
- **WHEN** the user selects a filamentcolors.xyz result with `hex_color: "FF6A00"`
- **THEN** the spool's color field is set to `#ff6a00`

#### Scenario: Selecting a swatch triggers an image download
- **WHEN** the user selects a filamentcolors.xyz result
- **THEN** the system requests the server to fetch and store that swatch's image and records the returned image reference on the spool form state

#### Scenario: Fields remain editable after selection
- **WHEN** the user selects a swatch and then edits the color field
- **THEN** the manually edited value is used on submission, not the swatch's original value

### Requirement: Server stores and serves downloaded swatch images
The server SHALL provide an endpoint that accepts a filamentcolors.xyz image URL, downloads the image, stores it on disk alongside the JSON data store, and returns a stable reference (id or path) that the client can use both to associate with a spool and to later retrieve the image. The server SHALL provide an endpoint to serve a previously stored image by its reference. An image already stored for a given source URL SHALL NOT be re-downloaded.

#### Scenario: New image is downloaded and stored
- **WHEN** the client requests storage of an image URL not previously seen
- **THEN** the server downloads the image, saves it to disk, and returns a new image reference

#### Scenario: Previously stored image is not re-downloaded
- **WHEN** the client requests storage of an image URL that was already downloaded
- **THEN** the server returns the existing image reference without a new download

#### Scenario: Stored image is served
- **WHEN** a client requests a stored image by its reference
- **THEN** the server responds with the image bytes and an appropriate content type

### Requirement: Swatch image displayed in place of flat color chip when available
Wherever a spool's color is rendered as a chip (spool list table, spool detail view), the system SHALL render the spool's stored swatch image instead of the flat color chip when a `swatch_image` reference is present on the spool. When absent, the existing flat color chip rendering SHALL be used unchanged. The `/colors` swatch grid's card fill is governed separately by the `spool-color-swatch-view` capability.

#### Scenario: List row shows swatch image
- **WHEN** a spool has a `swatch_image` reference and is rendered in the spool list table
- **THEN** the color cell displays the stored image instead of a flat color chip

#### Scenario: Detail view shows swatch image
- **WHEN** a spool has a `swatch_image` reference and is rendered in the spool detail view
- **THEN** the Colors field displays the stored image instead of a flat color chip

#### Scenario: Spool without swatch image is unaffected
- **WHEN** a spool has no `swatch_image` reference
- **THEN** it renders exactly as it did before this feature existed
