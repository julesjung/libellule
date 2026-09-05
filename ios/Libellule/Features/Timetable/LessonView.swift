//
//  LessonView.swift
//  Libellule
//
//  Created by Jules on 13/08/2026.
//

import SwiftUI
import LibelluleKit

struct LessonView: View {
    @Environment(\.colorScheme) var colorScheme
    let lesson: CachedLesson
    
    var body: some View {
        let start = DateFormatter.time.date(from: lesson.start)!
        let end = DateFormatter.time.date(from: lesson.end)!
        
        let teachers = lesson.teachers.joined(separator: ", ")
        let rooms = lesson.rooms.joined(separator: ", ")
        let groups = lesson.groups.joined(separator: ", ")
        
        GroupBox {
            HStack(spacing: 0) {
                VStack(alignment: .center, spacing: 12) {
                    Text(start.formatted(date: .omitted, time: .shortened))
                        .font(.caption)
                        .monospaced()
                    Spacer()
                    Text(end.formatted(date: .omitted, time: .shortened))
                        .font(.caption)
                        .monospaced()
                }
                .padding(.trailing)
                
                RoundedRectangle(cornerRadius: 10)
                    .foregroundStyle(Color(hex: lesson.background))
                    .frame(width: 8)
                
                VStack(alignment: .leading) {
                    Text(lesson.subjectName.localizedCapitalized)
                        .font(.headline)
                    if !teachers.isEmpty {
                        Label(teachers, systemImage: "person")
                            .font(.subheadline)
                    }
                    if !rooms.isEmpty {
                        Label(rooms, systemImage: "location")
                            .font(.subheadline)
                    }
                    if !groups.isEmpty {
                        Label(groups, systemImage: "person.3")
                            .font(.subheadline)
                    }
                }
                .padding(.leading)
                Spacer()
            }
        }
    }
}

#Preview {
    LessonView(lesson: CachedLesson(id: "29#pSS2zUe8bF2mLdD408rJsdOG_8GcbtV3b4wO8Q7fla4", start: "11:00:00", end: "12:00:00", subjectName: "PHYSIQUE-CHIMIE", teachers: ["LE CORRE T."], rooms: ["Salle 213 PHY"], groups: ["[1PH-CH1]"], background: "#EC6719"))
}
