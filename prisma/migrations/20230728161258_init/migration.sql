-- CreateTable
CREATE TABLE "Task" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "text" TEXT NOT NULL,
    "reminder" BOOLEAN NOT NULL,
    "timestamp" TEXT
);
