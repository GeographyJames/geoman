export function slugify(text: string): string {
  return text
    .toLowerCase()
    .normalize("NFKD")              // split accented characters
    .replace(/[\u0300-\u036f]/g, "") // remove accents
    .replace(/[^a-z0-9]+/g, "-")     // replace non-alphanumerics with -
    .replace(/^-+|-+$/g, "");        // trim leading/trailing -
}