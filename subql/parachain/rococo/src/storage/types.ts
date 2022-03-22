
interface CandidateReceipt {
  commitmentsHash: string,
  descriptor: ReceiptDescriptor,
}

interface ReceiptDescriptor {
  collator: string,
  erasureRoot: string,
  paraHead: string,
  paraId: number,
  persistedValidationDataHash: string,
  povHash: string,
  relayParent: string,
  signature: string,
  validationCodeHash: string,
}

