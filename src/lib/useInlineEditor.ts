import { useEffect, useState } from "react";

/**
 * Open/closed state for a block's inline editor.
 *
 * Learn mode has no editing chrome at all, so an editor left open in Author
 * mode must close the moment the mode flips — otherwise a textarea and a
 * "Done" button follow the reader into the reading view.
 */
export function useInlineEditor(authoring: boolean) {
  const [editing, setEditing] = useState(false);

  useEffect(() => {
    if (!authoring && editing) setEditing(false);
  }, [authoring, editing]);

  return [editing && authoring, setEditing] as const;
}
