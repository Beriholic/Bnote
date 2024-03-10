export function ExtractTitle(text: string): string {
  const regex = /#\s*(.*)/;
  const match = text.match(regex);

  if (match) {
    return match[1].trim();
  }
  return "未命名笔记";
}
