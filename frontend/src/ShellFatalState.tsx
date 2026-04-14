type ShellFatalStateProps = {
  message: string;
  serviceLabel: string;
  title: string;
  shellClassName: string;
  cardClassName: string;
  eyebrowClassName: string;
  messageClassName?: string;
  titleClassName?: string;
};

export function ShellFatalState({
  message,
  serviceLabel,
  title,
  shellClassName,
  cardClassName,
  eyebrowClassName,
  messageClassName = "text-secondary mb-0",
  titleClassName,
}: ShellFatalStateProps) {
  return (
    <main className={shellClassName}>
      <section className={cardClassName}>
        <p className={eyebrowClassName}>{serviceLabel}</p>
        <h1 className={titleClassName}>{title}</h1>
        <p className={messageClassName}>{message}</p>
      </section>
    </main>
  );
}
