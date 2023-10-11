pub struct UserToOrganizationAsCoach;

impl Linked for UserToOrganizationAsCoach {
    type FromEntity = User::Entity;

    type ToEntity = Organization::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            coaching_relationship::Relation::Coach.def().rev(),
            coaching_relationship::Relation::Organization.def(),
        ]  
    }
}

pub struct UserToOrganizationAsCoachee;

impl Linked for UserToOrganizationAsCoachee {
    type FromEntity = User::Entity;

    type ToEntity = Organization::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            coaching_relationship::Relation::Coachee.def().rev(),
            coaching_relationship::Relation::Organization.def(),
        ]  
    }
}

