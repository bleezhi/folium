package org.folium.paint

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.gestures.detectDragGestures
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.weight
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateListOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.StrokeCap
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.unit.dp

private data class Mark(val from: Offset, val to: Offset, val width: Float)

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            MaterialTheme {
                Surface(modifier = Modifier.fillMaxSize()) {
                    PaintScreen()
                }
            }
        }
    }
}

@Composable
private fun PaintScreen() {
    val marks = remember { mutableStateListOf<Mark>() }
    val currentColor = remember { mutableStateOf(Color(0xFF202124)) }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(Color(0xFFF4F3F0))
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp)
    ) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween
        ) {
            Column {
                Text("Folium", style = MaterialTheme.typography.headlineMedium)
                Text("Paint · early preview", style = MaterialTheme.typography.bodyMedium)
            }
            Button(onClick = { marks.clear() }) {
                Text("Clear")
            }
        }

        Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            listOf(Color(0xFF202124), Color(0xFFE45B4F), Color(0xFF3976C6), Color(0xFF39966B))
                .forEach { color ->
                    Button(onClick = { currentColor.value = color }) {
                        Text("●", color = color)
                    }
                }
        }

        Box(
            modifier = Modifier
                .weight(1f)
                .fillMaxWidth()
                .background(Color.White)
        ) {
            Canvas(
                modifier = Modifier
                    .fillMaxSize()
                    .pointerInput(currentColor.value) {
                        detectDragGestures(
                            onDragStart = { },
                            onDrag = { change, _ ->
                                val point = change.position
                                val previous = change.previousPosition
                                marks.add(
                                    Mark(
                                        from = previous,
                                        to = point,
                                        width = (change.pressure.coerceIn(0.2f, 1f) * 7f)
                                    )
                                )
                            }
                        )
                    }
            ) {
                marks.forEach { mark ->
                    drawLine(
                        color = currentColor.value,
                        start = mark.from,
                        end = mark.to,
                        strokeWidth = mark.width,
                        cap = StrokeCap.Round
                    )
                }
            }
        }

        Text(
            "A tiny first canvas. Draw with your finger or stylus.",
            style = MaterialTheme.typography.bodySmall
        )
    }
}
