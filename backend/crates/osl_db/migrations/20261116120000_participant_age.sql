-- Age from the result sheet at competition time.
ALTER TABLE "competition_participants"
    ADD COLUMN "age" SMALLINT CHECK ("age" > 0);
