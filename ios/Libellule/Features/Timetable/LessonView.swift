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

        NavigationLink {
            LessonDetails(lesson: lesson)
        } label: {
            HStack(spacing: 12) {
                RoundedRectangle(cornerRadius: 10)
                    .foregroundStyle(Color(hex: lesson.background))
                    .frame(width: 10)
                
                VStack(alignment: .leading) {
                    Text(start.formatted(date: .omitted, time: .shortened))
                        .font(.caption)
                        .monospaced()
                        .bold()
                    
                    Spacer()
                    
                    Text(end.formatted(date: .omitted, time: .shortened))
                        .font(.caption)
                        .monospaced()
                        .bold()
                }
                .frame(minWidth: 50)
                
                VStack(alignment: .leading) {
                    Text(lesson.subjectName.localizedCapitalized)
                        .font(.headline)
                    if !teachers.isEmpty {
                        Text(teachers)
                            .font(.subheadline)
                    }
                    if !rooms.isEmpty {
                        Text(rooms)
                            .font(.subheadline)
                    }
                }
                Spacer()
            }
        }
    }
}

#Preview {
    LessonView(lesson: CachedLesson(id: "29#pSS2zUe8bF2mLdD408rJsdOG_8GcbtV3b4wO8Q7fla4", start: "11:00:00", end: "12:00:00", subjectName: "PHYSIQUE-CHIMIE", teachers: ["LE CORRE T."], rooms: ["Salle 213 PHY"], groups: ["[1PH-CH1]"], background: "#EC6719"))
}
