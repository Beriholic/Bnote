import { invoke } from "@tauri-apps/api";

class Note {
  id: number;
  title: string;
  content: string;
  wordCount: number;
  created_at: string;

  constructor() {
    this.id = 0;
    this.title = "";
    this.content = "";
    this.wordCount = 0;
    this.created_at = "";
  }
  async save(): Promise<boolean> {
    let res = await invoke("create_note", {
      title: this.title,
      content: this.content,
      wordCnt: this.wordCount,
    });
    console.log(res);

    if (res) {
      return true;
    } else {
      return false;
    }
  }
}

export function new_note(
  id: number,
  title: string,
  content: string,
  wordCount: number,
  createdAt: string
) {
  let note = new Note();

  note.id = id;
  note.title = title;
  note.content = content;
  note.wordCount = wordCount;
  note.created_at = createdAt;
  return note;
}

export function save_note() {}

export default Note;
