import { Modal } from "./Modal";

interface Props {
  open: boolean;
  title: string;
  /** Spell out the exact consequence — no "are you sure?" without a because. */
  consequence: string;
  confirmLabel: string;
  destructive?: boolean;
  onConfirm: () => void;
  onCancel: () => void;
  children?: React.ReactNode;
}

export function ConfirmDialog({
  open,
  title,
  consequence,
  confirmLabel,
  destructive = true,
  onConfirm,
  onCancel,
  children,
}: Props) {
  return (
    <Modal
      open={open}
      onClose={onCancel}
      title={title}
      description={consequence}
      footer={
        <>
          <button className="btn" onClick={onCancel}>
            Cancel
          </button>
          <button
            className={destructive ? "btn btn-danger" : "btn btn-primary"}
            onClick={onConfirm}
          >
            {confirmLabel}
          </button>
        </>
      }
    >
      {children}
    </Modal>
  );
}
