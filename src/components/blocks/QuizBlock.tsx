import { useState } from "react";
import { Check, CircleHelp, Plus, Trash2, X } from "lucide-react";
import { BlockShell, EmptyCard } from "../BlockShell";
import { Markdown } from "../Markdown";
import { cx } from "@/lib/format";
import type { QuizQuestion } from "@/lib/types";
import { quizMeta, type BlockRendererProps } from "./common";
import { shellProps } from "./TextBlock";

const newId = () => `q${Date.now().toString(36)}${Math.random().toString(36).slice(2, 6)}`;

function blankQuestion(): QuizQuestion {
  return { id: newId(), prompt: "", choices: ["", ""], answerIndex: 0 };
}

export function QuizBlock(props: BlockRendererProps) {
  const { block, authoring, onPatch, onLearnEmpty } = props;
  const meta = quizMeta(block);
  const [editing, setEditing] = useState(false);
  const [answers, setAnswers] = useState<Record<string, number>>({});

  const write = (questions: QuizQuestion[]) =>
    onPatch({ metaJson: JSON.stringify({ ...meta, questions }) });

  const addQuestion = () => {
    write([...meta.questions, blankQuestion()]);
    setEditing(true);
  };

  const editor = (
    <div className="space-y-4">
      {meta.questions.map((q, qi) => (
        <div key={q.id} className="card space-y-3 p-4">
          <div className="flex items-start gap-2">
            <span className="mt-2 text-xs font-semibold text-ink-faint">{qi + 1}.</span>
            <input
              className="input"
              value={q.prompt}
              placeholder="What does this question ask?"
              onChange={(e) =>
                write(
                  meta.questions.map((x) =>
                    x.id === q.id ? { ...x, prompt: e.target.value } : x,
                  ),
                )
              }
            />
            <button
              className="btn btn-ghost btn-sm mt-1"
              title="Remove question"
              onClick={() => write(meta.questions.filter((x) => x.id !== q.id))}
            >
              <Trash2 size={14} />
            </button>
          </div>

          <div className="space-y-1.5 pl-6">
            {q.choices.map((choice, ci) => (
              <div key={ci} className="flex items-center gap-2">
                <button
                  className={cx(
                    "grid h-5 w-5 shrink-0 place-items-center rounded-full border",
                    q.answerIndex === ci
                      ? "border-ok bg-ok text-white"
                      : "border-line text-transparent",
                  )}
                  title="Mark as the correct answer"
                  onClick={() =>
                    write(
                      meta.questions.map((x) =>
                        x.id === q.id ? { ...x, answerIndex: ci } : x,
                      ),
                    )
                  }
                >
                  <Check size={12} />
                </button>
                <input
                  className="input"
                  value={choice}
                  placeholder={`Choice ${ci + 1}`}
                  onChange={(e) =>
                    write(
                      meta.questions.map((x) =>
                        x.id === q.id
                          ? {
                              ...x,
                              choices: x.choices.map((c, i) =>
                                i === ci ? e.target.value : c,
                              ),
                            }
                          : x,
                      ),
                    )
                  }
                />
                {q.choices.length > 2 && (
                  <button
                    className="btn btn-ghost btn-sm"
                    title="Remove choice"
                    onClick={() =>
                      write(
                        meta.questions.map((x) =>
                          x.id === q.id
                            ? {
                                ...x,
                                choices: x.choices.filter((_, i) => i !== ci),
                                answerIndex:
                                  x.answerIndex > ci
                                    ? x.answerIndex - 1
                                    : Math.min(x.answerIndex, x.choices.length - 2),
                              }
                            : x,
                        ),
                      )
                    }
                  >
                    <X size={13} />
                  </button>
                )}
              </div>
            ))}
            <button
              className="btn btn-ghost btn-sm"
              onClick={() =>
                write(
                  meta.questions.map((x) =>
                    x.id === q.id ? { ...x, choices: [...x.choices, ""] } : x,
                  ),
                )
              }
            >
              <Plus size={13} /> Add choice
            </button>
          </div>

          <input
            className="input ml-6 w-[calc(100%-1.5rem)]"
            value={q.explanation ?? ""}
            placeholder="Why that answer is right (optional, shown after answering)"
            onChange={(e) =>
              write(
                meta.questions.map((x) =>
                  x.id === q.id ? { ...x, explanation: e.target.value } : x,
                ),
              )
            }
          />
        </div>
      ))}
      <div className="flex items-center justify-between">
        <button className="btn btn-sm" onClick={addQuestion}>
          <Plus size={13} /> Add question
        </button>
        <button className="btn btn-sm" onClick={() => setEditing(false)}>
          Done
        </button>
      </div>
    </div>
  );

  const player = (
    <div className="space-y-4">
      {meta.questions.map((q, qi) => {
        const picked = answers[q.id];
        const answered = picked != null;
        return (
          <div key={q.id} className="card p-4">
            <p className="mb-3 text-sm font-medium text-ink">
              <span className="mr-2 text-ink-faint">{qi + 1}.</span>
              {q.prompt || <span className="text-ink-faint">(no question text)</span>}
            </p>
            <div className="space-y-1.5">
              {q.choices.map((choice, ci) => {
                const isAnswer = ci === q.answerIndex;
                const isPicked = picked === ci;
                return (
                  <button
                    key={ci}
                    disabled={answered}
                    onClick={() => setAnswers((a) => ({ ...a, [q.id]: ci }))}
                    className={cx(
                      "flex w-full items-center gap-2 rounded-lg border px-3 py-2 text-left text-sm transition-colors",
                      !answered && "border-line hover:border-brand hover:bg-surface-2",
                      answered && isAnswer && "border-ok bg-ok/10 text-ink",
                      answered && isPicked && !isAnswer && "border-danger bg-danger/10",
                      answered && !isPicked && !isAnswer && "border-line opacity-60",
                    )}
                  >
                    {answered && isAnswer && <Check size={14} className="text-ok" />}
                    {answered && isPicked && !isAnswer && (
                      <X size={14} className="text-danger" />
                    )}
                    <span>{choice || <span className="text-ink-faint">(empty)</span>}</span>
                  </button>
                );
              })}
            </div>
            {answered && q.explanation?.trim() && (
              <div className="mt-3 rounded-lg bg-surface-2 p-3 text-sm">
                <Markdown>{q.explanation}</Markdown>
              </div>
            )}
          </div>
        );
      })}
      <div className="flex gap-2">
        {Object.keys(answers).length > 0 && (
          <button className="btn btn-sm" onClick={() => setAnswers({})}>
            Try again
          </button>
        )}
        {authoring && (
          <button className="btn btn-sm" onClick={() => setEditing(true)}>
            Edit questions
          </button>
        )}
      </div>
    </div>
  );

  return (
    <BlockShell
      {...shellProps(props)}
      ready={editing && authoring ? editor : player}
      empty={
        editing && authoring ? (
          editor
        ) : (
          <EmptyCard
            authoring={authoring}
            icon={<CircleHelp size={26} />}
            title={authoring ? "No questions yet" : "Quiz coming soon"}
            subline={
              authoring
                ? "Add a few questions to check the idea landed."
                : "The questions for this section haven't been written yet."
            }
            captionMd={block.captionMd}
            action={authoring ? { label: "Add question", run: addQuestion } : undefined}
            onActivate={authoring ? addQuestion : onLearnEmpty}
          />
        )
      }
    />
  );
}
