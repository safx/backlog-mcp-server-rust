use backlog_issue::models::CustomFieldWithValue;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

#[cfg(feature = "writable")]
use backlog_issue::models::CustomFieldInput;
#[cfg(feature = "writable")]
use chrono::NaiveDate;

#[cfg(feature = "writable")]
use backlog_core::identifier::{
    CustomFieldId, CustomFieldItemId, IssueTypeId, PriorityId, ProjectId,
};
#[cfg(feature = "writable")]
use backlog_issue::api::AddIssueParamsBuilder;
#[cfg(feature = "writable")]
use std::collections::HashMap;

#[cfg(feature = "writable")]
fn benchmark_custom_field_serialization(c: &mut Criterion) {
    c.bench_function("serialize_100_custom_fields", |b| {
        let mut custom_fields = HashMap::new();

        // Create 100 custom fields with various types
        for i in 0..25 {
            let base_id = i * 4;
            custom_fields.insert(
                CustomFieldId::new(base_id + 1),
                CustomFieldInput::Text(format!("Text value {i}")),
            );
            custom_fields.insert(
                CustomFieldId::new(base_id + 2),
                CustomFieldInput::Numeric(i as f64 * std::f64::consts::PI),
            );
            custom_fields.insert(
                CustomFieldId::new(base_id + 3),
                CustomFieldInput::Date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
            );
            custom_fields.insert(
                CustomFieldId::new(base_id + 4),
                CustomFieldInput::MultipleList {
                    ids: vec![
                        CustomFieldItemId::new(i),
                        CustomFieldItemId::new(i + 100),
                        CustomFieldItemId::new(i + 200),
                    ],
                    other_value: None,
                },
            );
        }

        let params = AddIssueParamsBuilder::default()
            .project_id(ProjectId::new(1))
            .summary("Performance Test".to_string())
            .issue_type_id(IssueTypeId::new(1))
            .priority_id(PriorityId::new(2))
            .custom_fields(custom_fields)
            .build()
            .unwrap();

        b.iter(|| {
            let form_params: Vec<(String, String)> = black_box(&params).into();
            form_params
        });
    });

    c.bench_function("serialize_large_multiple_list", |b| {
        let mut custom_fields = HashMap::new();

        // Create a multiple list with 1000 items
        let large_ids: Vec<CustomFieldItemId> = (1..=1000).map(CustomFieldItemId::new).collect();
        custom_fields.insert(
            CustomFieldId::new(1),
            CustomFieldInput::MultipleList {
                ids: large_ids,
                other_value: Some("Other value".to_string()),
            },
        );

        let params = AddIssueParamsBuilder::default()
            .project_id(ProjectId::new(1))
            .summary("Large List Test".to_string())
            .issue_type_id(IssueTypeId::new(1))
            .priority_id(PriorityId::new(2))
            .custom_fields(custom_fields)
            .build()
            .unwrap();

        b.iter(|| {
            let form_params: Vec<(String, String)> = black_box(&params).into();
            form_params
        });
    });
}

fn benchmark_custom_field_deserialization(c: &mut Criterion) {
    c.bench_function("deserialize_text_field", |b| {
        let json = r#"{
            "id": 1,
            "fieldTypeId": 1,
            "name": "Text Field",
            "value": "Sample text value"
        }"#;

        b.iter(|| {
            let field: CustomFieldWithValue = serde_json::from_str(black_box(json)).unwrap();
            field
        });
    });

    c.bench_function("deserialize_multiple_list_field", |b| {
        let json = r#"{
            "id": 2,
            "fieldTypeId": 6,
            "name": "Multiple List",
            "value": [
                {"id": 1, "name": "Option 1"},
                {"id": 2, "name": "Option 2"},
                {"id": 3, "name": "Option 3"},
                {"id": 4, "name": "Option 4"},
                {"id": 5, "name": "Option 5"},
                {"id": 6, "name": "Option 6"},
                {"id": 7, "name": "Option 7"},
                {"id": 8, "name": "Option 8"},
                {"id": 9, "name": "Option 9"},
                {"id": 10, "name": "Option 10"}
            ],
            "otherValue": "Other description"
        }"#;

        b.iter(|| {
            let field: CustomFieldWithValue = serde_json::from_str(black_box(json)).unwrap();
            field
        });
    });

    c.bench_function("deserialize_50_custom_fields", |b| {
        let mut fields = Vec::new();
        for i in 0..50 {
            let field_json = match i % 4 {
                0 => format!(
                    r#"{{"id": {i}, "fieldTypeId": 1, "name": "Text {i}", "value": "Value {i}"}}"#
                ),
                1 => {
                    let value = i as f64 * 1.5;
                    format!(
                        r#"{{"id": {i}, "fieldTypeId": 3, "name": "Number {i}", "value": {value}}}"#
                    )
                }
                2 => {
                    let day = (i % 28) + 1;
                    format!(
                        r#"{{"id": {i}, "fieldTypeId": 4, "name": "Date {i}", "value": "2024-06-{day:02}"}}"#
                    )
                }
                _ => {
                    let id_times_10 = i * 10;
                    format!(
                        r#"{{"id": {i}, "fieldTypeId": 7, "name": "CheckBox {i}", "value": [{{"id": {id_times_10}, "name": "Check {i}"}}]}}"#
                    )
                }
            };
            fields.push(field_json);
        }
        let json = format!(r#"[{}]"#, fields.join(","));

        b.iter(|| {
            let fields: Vec<CustomFieldWithValue> = serde_json::from_str(black_box(&json)).unwrap();
            fields
        });
    });
}

#[cfg(feature = "writable")]
fn benchmark_custom_field_operations(c: &mut Criterion) {
    c.bench_function("to_form_value_operations", |b| {
        let inputs = vec![
            CustomFieldInput::Text("Sample text".to_string()),
            CustomFieldInput::TextArea("Multi\nLine\nText".to_string()),
            CustomFieldInput::Numeric(123.456),
            CustomFieldInput::Date(NaiveDate::from_ymd_opt(2024, 6, 24).unwrap()),
            CustomFieldInput::SingleList {
                id: CustomFieldItemId::new(100),
                other_value: Some("Other".to_string()),
            },
            CustomFieldInput::MultipleList {
                ids: vec![
                    CustomFieldItemId::new(1),
                    CustomFieldItemId::new(2),
                    CustomFieldItemId::new(3),
                    CustomFieldItemId::new(4),
                    CustomFieldItemId::new(5),
                ],
                other_value: None,
            },
            CustomFieldInput::CheckBox(vec![
                CustomFieldItemId::new(10),
                CustomFieldItemId::new(20),
                CustomFieldItemId::new(30),
            ]),
            CustomFieldInput::Radio {
                id: CustomFieldItemId::new(200),
                other_value: Some("Radio other".to_string()),
            },
        ];

        b.iter(|| {
            for input in &inputs {
                let (value, other) = black_box(input).to_form_value();
                black_box((value, other));
            }
        });
    });
}

#[cfg(feature = "writable")]
criterion_group!(
    benches,
    benchmark_custom_field_serialization,
    benchmark_custom_field_deserialization,
    benchmark_custom_field_operations
);

#[cfg(not(feature = "writable"))]
criterion_group!(benches, benchmark_custom_field_deserialization);

criterion_main!(benches);
