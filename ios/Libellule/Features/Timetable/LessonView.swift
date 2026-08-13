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
        let start = DateFormatter.datetime.date(from: lesson.start)!
        let end = DateFormatter.datetime.date(from: lesson.end)!
        
        NavigationLink {
            
        } label: {
            HStack {
                VStack {
                    Text(start.formatted(date: .omitted, time: .shortened))
                        .font(.caption)
                    Text(end.formatted(date: .omitted, time: .shortened))
                        .font(.caption)
                }
                .padding(.trailing)
                VStack(alignment: .leading) {
                    Text(lesson.subject.name)
                        .font(.headline)
                    Text(lesson.teachers.joined(separator: ", "))
                }
                Spacer()
            }
            .padding()
            .foregroundStyle(.foreground)
            .background(Color(hex: lesson.background)
                .brightness(colorScheme == .light ? 0.2 : -0.2)
            )
        }
    }
}

#Preview {
     LessonView(lesson: Lesson(id: "29#pSS2zUe8bF2mLdD408rJsdOG_8GcbtV3b4wO8Q7fla4", kind: 0, start: "2026-04-09 11:00:00", end: "2026-04-09 12:00:00", subject: LibelluleKit.Subject(id: "79#e3tr0OI2D45WgKUG94OSWX-dGhn6qbkA2Ck9m9r949c", name: "PHYSIQUE-CHIMIE"), teachers: ["LE CORRE T."], groups: [LibelluleKit.Group(id: "60#YB_uWYvBsWfNCCUQ2LX02V_e6RMqLlQEETXtEk-Cx4E", name: "[1PH-CH1]")], locations: [LibelluleKit.Location(id: "129#7cW1OV6DxNjy-8I2WDow9grWIMu8Jb0FLb_fbpDbRuc", name: "Salle 213 PHY")], background: "#EC6719"))
 }
 
