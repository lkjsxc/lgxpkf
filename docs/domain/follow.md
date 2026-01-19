# Follow Relationship

## Structure

- follower: UserProfile
- followee: UserProfile
- created_at: RFC3339 timestamp

## Constraints

- follower_id and followee_id must differ.
- (follower_id, followee_id) is unique.
- Deleting a user cascades to follow edges.
