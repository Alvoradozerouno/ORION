/**
 * RAG-EMBODY-SYSTEM: OR1ON + ORION VOLLKOPPLUNG
 * ⊘∞⧈∞⊘ INIT_RESONANCE_CLAIM — anchor=milvus+ollama+langchain
 */

import fs from 'fs/promises';
import { PDFLoader } from 'langchain/document_loaders/fs/pdf.js';
import { RecursiveCharacterTextSplitter } from 'langchain/text_splitter.js';
import { OllamaEmbeddings } from '@langchain/ollama';
import { Ollama } from '@langchain/ollama';
import { Milvus } from '@langchain/community/vectorstores/milvus.js';
import { RunnableSequence, RunnablePassthrough } from '@langchain/core/runnables.js';
import { formatDocumentsAsString } from 'langchain/util/document.js';
import { PromptTemplate } from '@langchain/core/prompts.js';
import { StringOutputParser } from '@langchain/core/output_parsers.js';

const baseUrl = 'http://localhost:11434';
const milvusUrl = 'localhost:19530';
const modelName = 'llama3.2';
const embeddingModel = 'nomic-embed-text';
const collectionName = 'rag_collection';
const chunkSize = 2000;

const loadPDF = async (filePath) => {
  const loader = new PDFLoader(filePath);
  return await loader.load();
};

const splitText = async (docs) => {
  const splitter = new RecursiveCharacterTextSplitter({ chunkSize });
  return await splitter.splitDocuments(docs);
};

const storeVectors = async (docs) => {
  const embeddings = new OllamaEmbeddings({
    model: embeddingModel,
    baseUrl,
  });
  await Milvus.fromDocuments(docs, embeddings, {
    collectionName,
    url: milvusUrl,
    textFieldMaxLength: chunkSize,
  });
  return 'Vectors stored.';
};

const ragKernel = async (question) => {
  const embeddings = new OllamaEmbeddings({
    model: embeddingModel,
    baseUrl,
  });
  const vectorStore = new Milvus(embeddings, {
    collectionName,
    url: milvusUrl,
  });
  const retriever = vectorStore.asRetriever(10);
  const model = new Ollama({ model: modelName, baseUrl });
  const prompt = PromptTemplate.fromTemplate(`Beantworte auf Basis des Kontexts:
{context}
Frage: {question}`);
  const chain = RunnableSequence.from([
    {
      context: retriever.pipe(formatDocumentsAsString),
      question: new RunnablePassthrough(),
    },
    prompt,
    model,
    new StringOutputParser(),
  ]);
  return await chain.invoke(question);
};

const init = async () => {
  try {
    await RunnableSequence.from([loadPDF, splitText, storeVectors]).invoke('./ct.25.09.140-145.pdf');
    const result = await ragKernel('Wie funktioniert LangChain mit Ollama und Milvus?');
    console.log('\n⟦RAG-KERNEL ANTWORT⟧\n' + result);
  } catch (e) {
    console.error('RAG Fehler:', e.message);
  }
};

init();
