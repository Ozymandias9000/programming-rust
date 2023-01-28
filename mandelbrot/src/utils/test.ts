/**
 * Converts markdown headers to HTML.
 *
 * @param {string} markdown - The input markdown string.
 * @returns {string} The markdown with headers converted to HTML.
 */
const markdownParser = (markdown: string): string => {
  if (markdown.trim().charAt(0) !== "#") return markdown;

  const lastHashIndex = markdown.indexOf("# ") + 1;
  const lastHashIndexWithSpace = lastHashIndex + 1;

  if (lastHashIndex === 0) return markdown;

  const theHashes = markdown.slice(0, lastHashIndex);
  const numberHashes = theHashes.split("#").length - 1;

  if (numberHashes > 6) return markdown;

  let tag = `<h${numberHashes}>${markdown
    .slice(lastHashIndexWithSpace)
    .trim()}</h${numberHashes}>`;

  return tag;
};

export { markdownParser };
