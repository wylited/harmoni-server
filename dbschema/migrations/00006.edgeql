CREATE MIGRATION m1t6jw6nbeomx7zqbupzudcn67sdtmzijsx5x6xf4azvosdhy3zdwq
    ONTO m1wsd6l7pg7pkgwyq573zxau63a7s7fyj7cwa66iuoyjrzqyezrs7q
{
  ALTER TYPE default::Client {
      ALTER PROPERTY secret {
          SET REQUIRED USING (<std::str>std::random());
      };
  };
};
