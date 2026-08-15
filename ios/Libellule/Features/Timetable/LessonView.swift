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
    let lesson: Lesson
    
    var body: some View {
        let start = DateFormatter.time.date(from: lesson.start)!
        let end = DateFormatter.time.date(from: lesson.end)!
        
        let teachers = lesson.teachers.joined(separator: ", ")
        let places = lesson.locations.map { $0.name } .joined(separator: ", ")
        let groups = lesson.groups.map { $0.name } .joined(separator: ", ")
        
        NavigationLink {
            
        } label: {
            HStack(spacing: 0) {
                VStack(spacing: 12) {
                    Text(start.formatted(date: .omitted, time: .shortened))
                        .font(.subheadline)
                    Text(end.formatted(date: .omitted, time: .shortened))
                        .font(.subheadline)
                }
                .padding(.trailing)
                VStack(alignment: .leading) {
                    Text(lesson.subject.name.localizedCapitalized)
                        .font(.headline)
                    if !teachers.isEmpty {
                        Label(teachers, systemImage: "person")
                            .font(.subheadline)
                    }
                    if !places.isEmpty {
                        Label(places, systemImage: "location")
                            .font(.subheadline)
                    }
                    if !groups.isEmpty {
                        Label(groups, systemImage: "person.3")
                            .font(.subheadline)
                    }
                }
                Spacer()
            }
            .padding()
            .foregroundStyle(.foreground)
            .background(Color(hex: lesson.background))
        }
    }
}

#Preview {
     LessonView(lesson: Lesson(id: "29#pSS2zUe8bF2mLdD408rJsdOG_8GcbtV3b4wO8Q7fla4", kind: 0, start: "2026-04-09 11:00:00", end: "2026-04-09 12:00:00", subject: LibelluleKit.Subject(id: "79#e3tr0OI2D45WgKUG94OSWX-dGhn6qbkA2Ck9m9r949c", name: "PHYSIQUE-CHIMIE"), teachers: ["LE CORRE T."], groups: [LibelluleKit.Group(id: "60#YB_uWYvBsWfNCCUQ2LX02V_e6RMqLlQEETXtEk-Cx4E", name: "[1PH-CH1]")], locations: [LibelluleKit.Location(id: "129#7cW1OV6DxNjy-8I2WDow9grWIMu8Jb0FLb_fbpDbRuc", name: "Salle 213 PHY")], background: "#EC6719"))
 }
 
