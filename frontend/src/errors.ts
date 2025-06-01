export class AppError extends Error {
  constructor(name: string, message: string) {
    super(message);
    this.name = name;
  }
}

export class WebProofError extends AppError {
  constructor(message: string) {
    super('WebProofError', message);
  }
}

export class UseChainError extends AppError {
  constructor(message: string) {
    super('UseChainError', message);
  }
}
