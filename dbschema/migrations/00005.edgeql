CREATE MIGRATION m1wsd6l7pg7pkgwyq573zxau63a7s7fyj7cwa66iuoyjrzqyezrs7q
    ONTO m1azc6mn4reqie2smm63zaglhed4cgxgof4nolzdofspj4zi4syoeq
{
  CREATE TYPE default::Client {
      CREATE PROPERTY secret: std::str;
  };
};
