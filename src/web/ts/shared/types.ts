type UnknownRecord = Record<string, unknown>;

export const isRecord = (value: unknown): value is UnknownRecord =>
  typeof value === "object" && value !== null;

export const isString = (value: unknown): value is string => typeof value === "string";

const isOptionalString = (value: unknown): value is string | null | undefined =>
  value === null || value === undefined || typeof value === "string";

export const isUserProfile = (value: unknown): value is LgxpkfUserProfile =>
  isRecord(value) &&
  isString(value.user_id) &&
  isString(value.email) &&
  (!("account_note_id" in value) || isOptionalString(value.account_note_id));

export const isNote = (value: unknown): value is LgxpkfNote =>
  isRecord(value) &&
  isString(value.id) &&
  isString(value.value) &&
  isString(value.created_at) &&
  (!("author" in value) || value.author === null || isUserProfile(value.author));

export const decodeNotes = (payload: unknown): LgxpkfNote[] => {
  if (Array.isArray(payload)) {
    return payload.filter(isNote);
  }
  if (isRecord(payload)) {
    const items = payload.items;
    const notes = payload.notes;
    if (Array.isArray(items)) return items.filter(isNote);
    if (Array.isArray(notes)) return notes.filter(isNote);
  }
  return [];
};

export const decodeAuthUser = (payload: unknown): LgxpkfUserProfile | null => {
  if (!isRecord(payload)) return null;
  const user = payload.user;
  return isUserProfile(user) ? user : null;
};

export const decodePostNote = (payload: unknown): LgxpkfNote | null => {
  if (!isRecord(payload)) return null;
  const root = payload.root;
  return isNote(root) ? root : null;
};

export const decodeFollowUserIds = (payload: unknown): string[] => {
  if (!isRecord(payload)) return [];
  const edges = payload.edges;
  if (!Array.isArray(edges)) return [];
  return edges.flatMap((edge) => {
    if (!isRecord(edge) || !isUserProfile(edge.user)) return [];
    return [edge.user.user_id];
  });
};

export type RelatedResponse = { center: LgxpkfNote; related: LgxpkfNote[] };

export const decodeRelatedResponse = (payload: unknown): RelatedResponse | null => {
  if (!isRecord(payload) || !isNote(payload.center)) return null;
  const related = Array.isArray(payload.related) ? payload.related : [];
  const notes = related.flatMap((entry) => {
    if (!isRecord(entry) || !isNote(entry.note)) return [];
    return [entry.note];
  });
  return { center: payload.center, related: notes };
};
