Where:
- `COURT_ID`: The ID of the court where the case is filed
- `YY`: The last two digits of the year when the case is filed
- `SEQUENCE`: A four-digit sequence number (padded with leading zeros if necessary)
- `JJ`: The initials of the assigned judge (or 'XX' if no judge is assigned)

Example: `5-23-0042-JD` (Court ID 5, filed in 2023, 42nd case, assigned to judge with initials JD)

## Generation Process

1. **Court ID**: Taken directly from the `court_id` provided when creating the case.

2. **Year**: Extracted from the current date using the `chrono` crate.

3. **Sequence**: A random 4-digit number between 1 and 9999, generated using the `rand` crate.

4. **Judge Initials**:
   - If a judge is assigned:
     - The judge's name is fetched from the database.
     - Initials are extracted from the name (first letter of each word, up to two letters).
     - If the name doesn't provide two initials, 'X' is used as padding.
   - If no judge is assigned, 'XX' is used.

## Implementation Details

### Key Functions

1. `create_case`:
   - Server function that handles the creation of a new case.
   - Fetches the judge's name if a judge ID is provided.
   - Calls `generate_case_number` to create the case number.
   - Inserts the new case into the database.

2. `generate_case_number`:
   - Takes `court_id` and `judge_name` as inputs.
   - Combines all parts to create the final case number.

3. `generate_judge_initials`:
   - Extracts initials from a judge's name.
   - Ensures the result is always two characters long.

### Database Interaction

- Uses the `spin_sdk::pg` module for PostgreSQL database operations.
- Fetches the judge's name using a SQL query if a judge ID is provided.
- Inserts the new case with the generated case number into the cases table.

### Error Handling

- The `create_case` function returns a `Result`, allowing for proper error handling.
- Database errors are caught and returned as `ServerFnError`s.

## Usage

The case number generation is automatically triggered when a new case is created through the user interface. Users don't need to manually enter or generate case numbers.

## Considerations

- The current implementation uses a random sequence number, which could theoretically lead to collisions. In a production environment, consider using a more robust method for generating unique sequence numbers.
- The system assumes that judge names are stored in the database and can be fetched using the judge's ID.
- Make sure to handle potential database errors gracefully in the UI.

## Future Improvements

- Implement a separate table or system for managing sequence numbers to ensure absolute uniqueness.
- Add functionality to update case numbers if a judge is assigned after initial case creation.
- Consider adding validation to ensure generated case numbers are unique before inserting into the database.
