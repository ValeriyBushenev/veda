use chrono::NaiveDateTime;
use futures::Future;
use std::ops::Add;
use tiberius::{BoxableIo, Error, Transaction};
use time::Duration;
use v_module::module::Module;
use v_onto::individual::*;
use voca_rs::chop;

pub const WINPAK_TIMEZONE: i64 = 3;
pub const CARD_NUMBER_FIELD_NAME: &str = "mnd-s:cardNumber";

pub const CARD_DATA_QUERY: &str = "\
SELECT [t1].[ActivationDate], [t1].[ExpirationDate], [t1].[RecordID],
concat([t2].[LastName],' ',[t2].[FirstName],' ',[t2].[Note1]) as Description,
[t2].[Note2] as TabNumber,
[t2].[Note17] as Birthday,
concat( [t2].[Note4]+' ',
    CASE WHEN [t2].[Note6]='0' THEN null ELSE [t2].[Note6]+' ' END,
    CASE WHEN [t2].[Note7]='0' THEN null ELSE [t2].[Note7]+' ' END,
    CASE WHEN [t2].[Note8]='0' THEN null ELSE [t2].[Note8] END) as Comment,
concat( CASE WHEN LTRIM([t2].[Note27])='' THEN null ELSE LTRIM([t2].[Note27]+CHAR(13)+CHAR(10)) END,
    CASE WHEN LTRIM([t2].[Note28])='' THEN null ELSE LTRIM([t2].[Note28]+CHAR(13)+CHAR(10)) END,
    CASE WHEN LTRIM([t2].[Note29])='' THEN null ELSE LTRIM([t2].[Note29]+CHAR(13)+CHAR(10)) END,
    CASE WHEN LTRIM([t2].[Note30])='' THEN null ELSE LTRIM([t2].[Note30]+CHAR(13)+CHAR(10)) END,
    CASE WHEN LTRIM([t2].[Note33])='' THEN null ELSE LTRIM([t2].[Note33]+CHAR(13)+CHAR(10)) END,
    CASE WHEN LTRIM([t2].[Note34])='' THEN null ELSE LTRIM([t2].[Note34]+CHAR(13)+CHAR(10)) END,
    CASE WHEN LTRIM([t2].[Note37])='' THEN null ELSE LTRIM([t2].[Note34]+CHAR(13)+CHAR(10)) END,
    CASE WHEN LTRIM([t2].[Note38])='' THEN null ELSE LTRIM([t2].[Note34]+CHAR(13)+CHAR(10)) END,
    CASE WHEN LTRIM([t2].[Note39])='' THEN null ELSE LTRIM([t2].[Note34]+CHAR(13)+CHAR(10)) END,
    CASE WHEN LTRIM([t2].[Note40])='' THEN null ELSE LTRIM([t2].[Note34]+CHAR(13)+CHAR(10)) END) as Equipment
FROM [WIN-PAK PRO].[dbo].[Card] t1
    JOIN [WIN-PAK PRO].[dbo].[CardHolder] t2 ON [t2].[RecordID]=[t1].[CardHolderID]
WHERE LTRIM([t1].[CardNumber])=@P1 and [t1].[deleted]=0 and [t2].[deleted]=0";

pub const ACCESS_LEVEL_QUERY: &str = "\
SELECT [t2].[AccessLevelID]
FROM [WIN-PAK PRO].[dbo].[Card] t1
    JOIN [WIN-PAK PRO].[dbo].[CardAccessLevels] t2 ON [t2].[CardID]=[t1].[RecordID]
WHERE LTRIM([t1].[CardNumber])=@P1 and [t1].[deleted]=0 and [t2].[deleted]=0";

// CLEAR CARD

pub const CLEAR_CARD: &str = "\
UPDATE [WIN-PAK PRO].[dbo].[Card]
SET [Deleted]=1,[CardStatus]=0,[AccessLevelID]=0
WHERE LTRIM([CardNumber])=@P1 and [Deleted]=0";

pub fn clear_card<I: BoxableIo + 'static>(card_number: String, transaction: Transaction<I>) -> Box<dyn Future<Item = Transaction<I>, Error = Error>> {
    Box::new(transaction.exec(CLEAR_CARD, &[&card_number.as_str()]).and_then(|(_result, trans)| Ok(trans)))
}

// INSERT CARD

const INSERT_CARD: &str = "\
INSERT INTO [WIN-PAK PRO].[dbo].[Card]
(AccountID,TimeStamp,UserID,NodeId,Deleted,UserPriority,CardNumber,Issue,CardHolderID,AccessLevelID,ActivationDate,ExpirationDate,NoOfUsesLeft,CMDFileID,
CardStatus,Display,BackDrop1ID,BackDrop2ID,ActionGroupID,LastReaderHID,PrintStatus,SpareW1,SpareW2,SpareW3,SpareW4,SpareDW1,SpareDW2,SpareDW3,SpareDW4)
VALUES (1,@P1,0,0,0,0,@P2,0,@P5,-1,@P3,@P4,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0)";

pub fn insert_card<I: BoxableIo + 'static>(
    now: NaiveDateTime,
    card_number: String,
    date_from: Option<i64>,
    date_to: Option<i64>,
    id: i32,
    transaction: Transaction<I>,
) -> Box<dyn Future<Item = Transaction<I>, Error = Error>> {
    Box::new(
        transaction
            .exec(
                INSERT_CARD,
                &[
                    &now,
                    &card_number.as_str(),
                    &NaiveDateTime::from_timestamp(date_from.unwrap_or_default(), 0).add(Duration::hours(WINPAK_TIMEZONE)),
                    &NaiveDateTime::from_timestamp(date_to.unwrap_or_default(), 0).add(Duration::hours(WINPAK_TIMEZONE)),
                    &id,
                ],
            )
            .and_then(|(_result, trans)| Ok(trans)),
    )
}

// INSERT CARD HOLDER
const INSERT_VEHICLE_HOLDER: &str = "\
INSERT INTO [WIN-PAK PRO].[dbo].[CardHolder]
(AccountID,TimeStamp,UserID,NodeID,Deleted,UserPriority,LastName,Note3,Note4,Note5,Note6,Note11,Note16,Note18,Note19,Note22,Note32,Note24)
VALUES(1,@P1,0,0,0,0,@P2,'183','ТРАНСПОРТ',@P3,@P4,'0',@P5,@P6,@P7,@P8,@P9,@P10)";

const INSERT_HUMAN_CARDHOLDER: &str = "\
INSERT INTO [WIN-PAK PRO].[dbo].[CardHolder]
(AccountID,TimeStamp,UserId,NodeId,Deleted,UserPriority,FirstName,LastName,Note1,Note2,Note3,Note4,Note5,Note6,
Note7,Note8,Note11,Note15,Note16,Note17,Note19,Note22,Note32,Note24)
VALUES(1,@P1,0,0,0,0,@P2,@P3,@P4,@P5,@P6,@P7,'0',null,@P8,@P9,'0',@P10,@P11,@P12,@P13,@P14,@P15,@P16)";

pub fn insert_card_holder<I: BoxableIo + 'static>(
    id: &str,
    now: NaiveDateTime,
    is_vehicle: bool,
    is_human: bool,
    module: &mut Module,
    card_number: String,
    indv: &mut Individual,
    transaction: Transaction<I>,
) -> Box<dyn Future<Item = Transaction<I>, Error = Error>> {
    if is_vehicle {
        let label = indv.get_first_literal("mnd-s:passVehicleRegistrationNumber").unwrap_or_default()
            + " "
            + module.get_literal_of_link(indv, "v-s:hasVehicleModel", "rdfs:label", &mut Individual::default()).unwrap_or_default().as_str();

        Box::new(
            transaction
                .exec(
                    INSERT_VEHICLE_HOLDER,
                    &[
                        &now,
                        &label.as_str(),
                        &module.get_literal_of_link(indv, "v-s:supplier", "v-s:taxId", &mut Individual::default()).unwrap_or_default().as_str(),
                        &module.get_literal_of_link(indv, "v-s:supplier", "v-s:shortLabel", &mut Individual::default()).unwrap_or_default().as_str(),
                        &label.as_str(),
                        &label.as_str(),
                        &indv.get_first_literal("rdfs:comment").unwrap_or_default().as_str(),
                        &module.get_datetime_of_link(indv, "v-s:parent", "v-s:registrationDate", &mut Individual::default()).unwrap_or_default(),
                        &card_number.as_str(),
                        &id,
                    ],
                )
                .and_then(|(_result, trans)| Ok(trans)),
        )
    } else if is_human {
        let mut first_name = String::new();
        let mut last_name = String::new();
        let mut middle_name = String::new();
        let mut tab_number = String::new();
        let mut birthday = 0;
        let mut occupation = String::new();
        let mut icp = Individual::default();

        if let Some(cp) = indv.get_first_literal("v-s:correspondentPerson") {
            if module.get_individual(&cp, &mut icp).is_some() {
                if let Some(employee) = module.get_individual(&mut icp.get_first_literal("v-s:employee").unwrap_or_default(), &mut Individual::default()) {
                    first_name = employee.get_first_literal("v-s:firstName").unwrap_or_default();
                    last_name = employee.get_first_literal("v-s:lastName").unwrap_or_default();
                    middle_name = employee.get_first_literal("v-s:middleName").unwrap_or_default();
                    tab_number = employee.get_first_literal("v-s:tabNumber").unwrap_or_default();
                    birthday = employee.get_first_datetime("v-s:birthday").unwrap_or_default();
                }
                occupation = module.get_literal_of_link(&mut icp, "v-s:occupation", "v-s:title", &mut Individual::default()).unwrap_or_default();
            }
        } else {
            first_name = indv.get_first_literal("mnd-s:passFirstName").unwrap_or_default();
            last_name = indv.get_first_literal("mnd-s:passLastName").unwrap_or_default();
            middle_name = indv.get_first_literal("mnd-s:passMiddleName").unwrap_or_default();
            birthday = indv.get_first_datetime("v-s:birthday").unwrap_or_default();
            occupation = indv.get_first_literal("mnd-s:passPosition").unwrap_or_default();
        }

        Box::new(
            transaction
                .exec(
                    INSERT_HUMAN_CARDHOLDER,
                    &[
                        &now,
                        &first_name.as_str(),
                        &last_name.as_str(),
                        &middle_name.as_str(),
                        &tab_number.as_str(),
                        &module.get_literal_of_link(indv, "v-s:correspondentOrganization", "v-s:taxId", &mut Individual::default()).unwrap_or_default().as_str(),
                        &module.get_literal_of_link(indv, "v-s:supplier", "v-s:shortLabel", &mut Individual::default()).unwrap_or_default().as_str(),
                        &module.get_literal_of_link(&mut icp, "v-s:parentUnit", "rdfs:label", &mut Individual::default()).unwrap_or_default().as_str(),
                        &occupation.as_str(),
                        &last_name.as_str(),
                        &first_name.as_str(),
                        &NaiveDateTime::from_timestamp(birthday, 0).add(Duration::hours(WINPAK_TIMEZONE)).format("%Y-%m-%d").to_string().as_str(),
                        &indv.get_first_literal("rdfs:comment").unwrap_or_default().as_str(),
                        &NaiveDateTime::from_timestamp(
                            module.get_datetime_of_link(indv, "v-s:parent", "v-s:registrationDate", &mut Individual::default()).unwrap_or_default(),
                            0,
                        )
                        .format("%Y-%m-%d")
                        .to_string()
                        .as_str(),
                        &card_number.as_str(),
                        &id,
                    ],
                )
                .and_then(|(_result, trans)| Ok(trans)),
        )
    } else {
        Box::new(transaction.simple_exec("").and_then(|(_, trans)| Ok(trans)))
    }
}

// UPDATE CARD DATE

pub const UPDATE_CARD_DATE: &str = "\
UPDATE [WIN-PAK PRO].[dbo].[Card]
    SET [ActivationDate]=@P1, [ExpirationDate]=@P2
    WHERE LTRIM([CardNumber])=@P3 and [deleted]=0";

pub fn update_card_date<I: BoxableIo + 'static>(
    date_from: Option<i64>,
    date_to: Option<i64>,
    card_number: String,
    transaction: Transaction<I>,
) -> Box<dyn Future<Item = Transaction<I>, Error = Error>> {
    if date_to.is_some() && date_from.is_some() {
        Box::new(
            transaction
                .exec(
                    UPDATE_CARD_DATE,
                    &[
                        &NaiveDateTime::from_timestamp(date_from.unwrap(), 0).add(Duration::hours(WINPAK_TIMEZONE)),
                        &NaiveDateTime::from_timestamp(date_to.unwrap(), 0).add(Duration::hours(WINPAK_TIMEZONE)),
                        &card_number.as_str(),
                    ],
                )
                .and_then(|(_result, trans)| Ok(trans)),
        )
    } else {
        Box::new(transaction.simple_exec("").and_then(|(_, trans)| Ok(trans)))
    }
}

// CLEAR ACCESS LEVEL

pub const CLEAR_ACCESS_LEVEL: &str = "\
UPDATE t1
   SET [t1].[Deleted]=1
FROM [WIN-PAK PRO].[dbo].[CardAccessLevels] t1
    JOIN [WIN-PAK PRO].[dbo].[Card] t2 ON [t2].[RecordID]=[t1].[CardID]
WHERE LTRIM([t2].[CardNumber])=@P1 and [t2].[CardHolderID]<>0 and [t1].[deleted]=0 and [t2].[deleted]=0";

pub fn clear_access_level<I: BoxableIo + 'static>(card_number: String, transaction: Transaction<I>) -> Box<dyn Future<Item = Transaction<I>, Error = Error>> {
    Box::new(transaction.exec(CLEAR_ACCESS_LEVEL, &[&card_number.as_str()]).and_then(|(_result, trans)| Ok(trans)))
}

// INSERT ACCESS LEVEL

pub const INSERT_ACCESS_LEVEL: &str = "\
INSERT INTO [WIN-PAK PRO].[dbo].[CardAccessLevels]  (AccountID,TimeStamp,UserID,NodeID,Deleted,UserPriority,CardID,AccessLevelID,SpareW1,SpareW2,SpareW3,SpareW4,SpareDW1,SpareDW2,SpareDW3,SpareDW4)
VALUES (0,@P1,0,0,0,0,
    (SELECT RecordID FROM [WIN-PAK PRO].[dbo].[Card] WHERE LTRIM([CardNumber])=@P2 and [Deleted]=0),
    @P3,0,0,0,0,0,0,0,0)";

pub fn update_access_level<I: BoxableIo + 'static>(
    now: NaiveDateTime,
    idx: usize,
    levels: Vec<String>,
    card_number: String,
    transaction: Transaction<I>,
) -> Box<dyn Future<Item = Transaction<I>, Error = Error>> {
    if idx < levels.len() {
        Box::new(
            transaction
                .exec(INSERT_ACCESS_LEVEL, &[&now, &card_number.as_str(), &levels.get(idx).unwrap().as_str()])
                .and_then(|(_result, trans)| Ok(trans))
                .and_then(move |trans| update_access_level(now, idx + 1, levels, card_number, trans)),
        )
    } else {
        Box::new(transaction.simple_exec("").and_then(|(_, trans)| Ok(trans)))
    }
}

// UPDATE EQUIPMENT

pub const UPDATE_EQUIPMENT: &str = "\
UPDATE t1 SET
    [t1].[Note27] = @P1, [t1].[Note28] = @P2, [t1].[Note29] = @P3, [t1].[Note30] = @P4, [t1].[Note33] = @P5,
    [t1].[Note34] = @P6, [t1].[Note37] = @P7, [t1].[Note38] = @P8, [t1].[Note39] = @P9, [t1].[Note40] = @P10
FROM [WIN-PAK PRO].[dbo].[CardHolder] t1
JOIN [WIN-PAK PRO].[dbo].[Card] t2 ON [t2].[CardHolderID]=[t1].[RecordId]
WHERE LTRIM([t2].[CardNumber])=@P11 and [t2].[CardHolderID]<>0 and [t1].[deleted]=0 and [t2].[deleted]=0";

pub fn update_equipment<I: BoxableIo + 'static>(
    values: Vec<String>,
    card_number: String,
    transaction: Transaction<I>,
) -> Box<dyn Future<Item = Transaction<I>, Error = Error>> {
    let mut tv: Vec<&str> = Vec::new();
    for idx in 0..10 {
        if let Some(s) = values.get(idx) {
            tv.push(s.as_str());
        } else {
            tv.push("");
        }
    }

    Box::new(
        transaction
            .exec(UPDATE_EQUIPMENT, &[&tv[0], &tv[1], &tv[2], &tv[3], &tv[4], &tv[5], &tv[6], &tv[7], &tv[8], &tv[9], &card_number.as_str()])
            .and_then(|(_result, trans)| Ok(trans)),
    )
}

pub const UPDATE_EQUIPMENT_WHERE_ID: &str = "\
UPDATE t1 SET
    [t1].[Note27] = @P1, [t1].[Note28] = @P2, [t1].[Note29] = @P3, [t1].[Note30] = @P4, [t1].[Note33] = @P5,
    [t1].[Note34] = @P6, [t1].[Note37] = @P7, [t1].[Note38] = @P8, [t1].[Note39] = @P9, [t1].[Note40] = @P10
FROM [WIN-PAK PRO].[dbo].[CardHolder] t1
   WHERE [RecordID]=@P11";

pub fn update_equipment_where_id<I: BoxableIo + 'static>(
    values: Vec<String>,
    id: i32,
    transaction: Transaction<I>,
) -> Box<dyn Future<Item = Transaction<I>, Error = Error>> {
    let mut tv: Vec<&str> = Vec::new();
    for idx in 0..10 {
        if let Some(s) = values.get(idx) {
            tv.push(s.as_str());
        } else {
            tv.push("");
        }
    }

    Box::new(
        transaction
            .exec(UPDATE_EQUIPMENT_WHERE_ID, &[&tv[0], &tv[1], &tv[2], &tv[3], &tv[4], &tv[5], &tv[6], &tv[7], &tv[8], &tv[9], &id])
            .and_then(|(_result, trans)| Ok(trans)),
    )
}

pub fn split_str_for_winpak_db_columns(src: &str, len: usize, res: &mut Vec<String>) {
    for el in src.split('\n') {
        let mut start = 0;
        let mut end = len;
        loop {
            if end >= el.len() {
                end = el.len();
            }

            let ss = chop::substring(el, start, end);
            if !ss.is_empty() {
                res.push(chop::substring(el, start, end));
            } else {
                break;
            }

            if end >= el.len() {
                break;
            }
            start = end;
            end += len;
        }
    }
}

pub fn get_access_level(indv: &mut Individual, access_levels: &mut Vec<String>) {
    if let Some(access_levels_uris) = indv.get_literals("mnd-s:hasAccessLevel") {
        for l in access_levels_uris {
            if let Some(nl) = l.rsplit("_").next() {
                access_levels.push(nl.to_string());
            }
        }
    }
}

pub fn get_equipment_list(indv: &mut Individual, list: &mut Vec<String>) {
    if let Some(pass_equipment) = indv.get_first_literal("mnd-s:passEquipment") {
        split_str_for_winpak_db_columns(&pass_equipment, 64, list);
    }
}
