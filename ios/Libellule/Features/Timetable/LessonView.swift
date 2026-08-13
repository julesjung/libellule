//
//  LessonView.swift
//  Libellule
//
//  Created by Jules on 13/08/2026.
//

import SwiftUI
import LibelluleKit

struct LessonView: View {
    let lesson: Lesson
    
    var body: some View {
        let start = DateFormatter.datetime.date(from: lesson.start)!
        
        VStack {
            HStack {
                Text(start.formatted(date: .omitted, time: .shortened))
                Spacer()
            }
            Text(lesson.subject.name)
                .font(.headline)
            Text(lesson.teachers.joined(separator: ", "))
        }
        .padding()
        .background(RoundedRectangle(cornerRadius: 10, style: .continuous).fill(Color(hex: lesson.background)))
    }
}

#Preview {
     LessonView(lesson: Lesson(id: "29#pSS2zUe8bF2mLdD408rJsdOG_8GcbtV3b4wO8Q7fla4", kind: 0, start: "2026-04-09T11:00:00.000000000", end: "2026-04-09T12:00:00.000000000", subject: LibelluleKit.Subject(id: "79#e3tr0OI2D45WgKUG94OSWX-dGhn6qbkA2Ck9m9r949c", name: "PHYSIQUE-CHIMIE"), teachers: ["LE CORRE T."], groups: [LibelluleKit.Group(id: "60#YB_uWYvBsWfNCCUQ2LX02V_e6RMqLlQEETXtEk-Cx4E", name: "[1PH-CH1]")], locations: [LibelluleKit.Location(id: "129#7cW1OV6DxNjy-8I2WDow9grWIMu8Jb0FLb_fbpDbRuc", name: "Salle 213 PHY")], background: "#EC6719"))
 }
 
