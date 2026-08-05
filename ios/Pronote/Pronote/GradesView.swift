//
//  GradesView.swift
//  Pronote
//
//  Created by Jules on 20/07/2026.
//

import SwiftUI
import PronoteKit

struct GradesView: View {
    var gradesData: GradesData
    
    var body: some View {
        Picker("Période", selection: .constant(1)) {
            Text("Trimestre 1")
                .tag(1)
            Text("Trimestre 2")
                .tag(2)
            Text("Trimestre 3")
                .tag(3)
        }
        .pickerStyle(.menu)
        .buttonStyle(.glass)
        List(gradesData.assignments, id: \.self) { assignment in
            HStack {
                VStack(alignment: .leading) {
                    Text(assignment.subject.name).lineLimit(1)
                    Text(assignment.label).font(.caption).lineLimit(1)
                }
                Spacer()
                Text("\(assignment.grade))/\(Int(assignment.scale))")
            }
        }
        .task {
            print(gradesData)
        }
        .listStyle(.plain)
        .navigationTitle(Text("Notes"))
    }
}

#Preview {
    GradesView(gradesData: GradesData(subjects: [], assignments: []))
}
