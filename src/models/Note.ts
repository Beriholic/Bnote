class Note {
  id: number;
  title: string;
  content: string;
  createdAt: string;

  constructor(id: number, title: string, content: string, createdAt: string) {
    this.id = id;
    this.title = title;
    this.content = content;
    this.createdAt = createdAt;
  }
}
export default Note;
